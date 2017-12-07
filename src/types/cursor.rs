//! Wrapper for wlr_cursor

use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;
use types::input_device::InputDevice;
use types::output::OutputLayout;

use wlroots_sys::{wlr_cursor, wlr_cursor_attach_output_layout, wlr_cursor_create,
                  wlr_cursor_destroy, wlr_cursor_move, wlr_cursor_warp};

#[derive(Debug)]
pub struct Cursor {
    cursor: *mut wlr_cursor,
    layout: Option<Rc<RefCell<OutputLayout>>>
}

impl Cursor {
    pub fn new() -> Option<Cursor> {
        unsafe {
            let cursor = wlr_cursor_create();
            if cursor.is_null() {
                None
            } else {
                Some(Cursor {
                         cursor: cursor,
                         layout: None
                     })
            }
        }
    }

    pub fn coords(&self) -> (f64, f64) {
        unsafe { ((*self.cursor).x, (*self.cursor).y) }
    }

    pub fn warp(&mut self, dev: Option<InputDevice>, x: f64, y: f64) -> bool {
        unsafe {
            let dev_ptr = dev.map(|dev| dev.to_ptr()).unwrap_or(ptr::null_mut());
            wlr_cursor_warp(self.cursor, dev_ptr, x, y)
        }
    }

    /// Attaches an output layout to the cursor.
    /// The layout specifies the boundaries of the cursor, i.e where it can go.
    pub fn attach_output_layout(&mut self, layout: Rc<RefCell<OutputLayout>>) {
        unsafe {
            // NOTE Rationale for why the pointer isn't leaked from the refcell:
            // * A pointer is not stored to the layout, the internal state is just updated.
            wlr_cursor_attach_output_layout(self.cursor, layout.borrow_mut().to_ptr());
            self.layout = Some(layout);
        }
    }

    pub fn move_to(&mut self, dev: &InputDevice, delta_x: f64, delta_y: f64) {
        unsafe { wlr_cursor_move(self.cursor, dev.to_ptr(), delta_x, delta_y) }
    }

    pub fn output_layout(&self) -> &Option<Rc<RefCell<OutputLayout>>> {
        &self.layout
    }
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe { wlr_cursor_destroy(self.cursor) }
    }
}

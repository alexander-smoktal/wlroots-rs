//! Wrapper for wlr_cursor

use std::cell::RefCell;
use std::ptr;
use std::rc::Rc;
use types::input_device::InputDevice;
use types::output::{OutputHandle, OutputLayout};
use types::pointer::PointerHandle;
use types::xcursor::XCursorImage;

use wlroots_sys::{wlr_cursor, wlr_cursor_attach_output_layout, wlr_cursor_create,
                  wlr_cursor_destroy, wlr_cursor_map_to_output, wlr_cursor_attach_input_device, wlr_cursor_move,
                  wlr_cursor_set_image, wlr_cursor_warp};

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

    pub fn set_image(&mut self,
                     pixels: &[u8],
                     stride: i32,
                     width: u32,
                     height: u32,
                     hotspot_x: i32,
                     hotspot_y: i32,
                     scale: u32) {
        unsafe {
            wlr_cursor_set_image(self.cursor,
                                 pixels.as_ptr(),
                                 stride,
                                 width,
                                 height,
                                 hotspot_x,
                                 hotspot_y,
                                 scale)
        }
    }

    pub fn set_xcursor_image(&mut self, image: &XCursorImage) {
        let size = image.size();
        let hotspots = image.hotspots();
        self.set_image(image.pixels(),
                       size.0 as i32,
                       size.0,
                       size.1,
                       hotspots.0 as i32,
                       hotspots.1 as i32,
                       1u32)
    }

    pub fn map_to_output(&self, output: Option<&mut OutputHandle>) {
        unsafe { wlr_cursor_map_to_output(self.cursor, output.map_or(ptr::null_mut(), |out| out.to_ptr())) }
    }

    pub fn attach_input_device(&self, input: &mut PointerHandle) {
        unsafe { wlr_cursor_attach_input_device(self.cursor, input.input_device()) }
    }
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe { wlr_cursor_destroy(self.cursor) }
    }
}

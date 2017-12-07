//! Wrapper for wlr_xcursor

use std::slice;
use std::marker::PhantomData;
use wlroots_sys::{wlr_xcursor, wlr_xcursor_frame, wlr_xcursor_image};

#[derive(Debug)]
pub struct XCursor<'cursor> {
    xcursor: *mut wlr_xcursor,
    _phantom: PhantomData<&'cursor ()>
}

impl<'cursor> XCursor<'cursor> {
    pub(crate) unsafe fn new(xcursor: *mut wlr_xcursor) -> Self {
        XCursor {
            xcursor,
            _phantom: PhantomData
        }
    }

    pub fn frame(&self, time: u32) -> i32 {
        unsafe { wlr_xcursor_frame(self.xcursor, time) }
    }

    pub fn images(&self) -> Vec<XCursorImage<'cursor>> {
        unsafe {
            let cursors_slice = slice::from_raw_parts_mut((*self.xcursor).images,
                                                          (*self.xcursor).image_count as usize);
            cursors_slice
                .iter()
                .map(|cursor| XCursorImage::new(*cursor))
                .collect()
        }
    }
}

#[derive(Debug)]
pub struct XCursorImage<'image> {
    image: *const wlr_xcursor_image,
    _phantom: PhantomData<&'image ()>
}

impl<'image> XCursorImage<'image> {
    pub(crate) unsafe fn new(image: *const wlr_xcursor_image) -> Self {
        XCursorImage {
            image,
            _phantom: PhantomData
        }
    }
}

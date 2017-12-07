//! Wrapper for wlr_xcursor_theme

use std::{ptr, slice};
use std::ffi::CStr;
use types::xcursor::XCursor;
use utils::safe_as_cstring;
use wlroots_sys::{wlr_xcursor_theme, wlr_xcursor_theme_destroy, wlr_xcursor_theme_get_cursor,
                  wlr_xcursor_theme_load};

#[derive(Debug)]
pub struct XCursorTheme {
    theme: *mut wlr_xcursor_theme
}

impl XCursorTheme {
    /// If no name is given, defaults to "default".
    pub fn load(name: Option<String>, size: i32) -> Option<Self> {
        unsafe {
            let name_ptr = name.map_or(ptr::null(), |s| safe_as_cstring(s).as_ptr());
            let theme = wlr_xcursor_theme_load(name_ptr, size);

            if theme.is_null() {
                None
            } else {
                Some(XCursorTheme { theme })
            }
        }
    }

    pub fn get_cursor<'theme>(&'theme self, name: String) -> Option<XCursor<'theme>> {
        let xcursor =
            unsafe { wlr_xcursor_theme_get_cursor(self.theme, safe_as_cstring(name).as_ptr()) };

        if xcursor.is_null() {
            None
        } else {
            unsafe { Some(XCursor::new(xcursor)) }
        }
    }

    pub fn cursor_count(&self) -> u32 {
        unsafe { (*self.theme).cursor_count }
    }

    pub fn name(&self) -> String {
        unsafe {
            CStr::from_ptr((*self.theme).name)
                .to_str()
                .unwrap_or("")
                .into()
        }
    }

    pub fn size(&self) -> i32 {
        unsafe { (*self.theme).size }
    }

    pub fn cursors<'theme>(&'theme self) -> Vec<XCursor<'theme>> {
        unsafe {
            let cursor_slice = slice::from_raw_parts_mut((*self.theme).cursors,
                                                         self.cursor_count() as usize);

            cursor_slice
                .iter_mut()
                .map(|xcursor| XCursor::new(*xcursor))
                .collect()
        }
    }
}

impl Drop for XCursorTheme {
    fn drop(&mut self) {
        unsafe { wlr_xcursor_theme_destroy(self.theme) }
    }
}

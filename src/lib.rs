use std::ffi::{c_char, CString};
use x11::xfixes::XFixesHideCursor;
use x11::xlib::{Display, XDefaultRootWindow, XOpenDisplay, XSync};

pub fn x_default_root_window(display: &mut Display) -> u64 {
    unsafe { XDefaultRootWindow(display) }
}

pub fn x_fixes_hide_cursor(display: &mut Display, window: u64) {
    unsafe { XFixesHideCursor(display, window) }
}

pub fn x_open_display<'a>(name: impl Into<String>) -> Option<&'a mut Display> {
    match CString::new(name.into()) {
        Ok(name) => unsafe { XOpenDisplay(name.as_ptr() as *const c_char).as_mut() },
        Err(_) => None,
    }
}

pub fn x_sync(display: &mut Display, discard: bool) {
    unsafe {
        XSync(display, discard as i32);
    }
}

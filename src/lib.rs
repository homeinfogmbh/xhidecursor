use std::ffi::{c_char, CString};
use x11::xfixes::XFixesHideCursor;
use x11::xlib::{Display, XDefaultRootWindow, XOpenDisplay, XSync};

pub fn x_hide_cursor(display: &mut Display) {
    unsafe {
        XFixesHideCursor(display, XDefaultRootWindow(display));
        XSync(display, 1);
    }
}

pub fn x_open_display<'a>(name: impl Into<String>) -> Option<&'a mut Display> {
    match CString::new(name.into()) {
        Ok(name) => unsafe { XOpenDisplay(name.as_ptr() as *const c_char).as_mut() },
        Err(_) => None,
    }
}

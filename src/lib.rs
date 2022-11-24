use std::ffi::{c_char, CString};
use x11::xfixes::XFixesHideCursor;
use x11::xlib::{XDefaultRootWindow, XOpenDisplay, XSync};

pub fn x_hide_cursor(display_name: &CString) -> bool {
    let display;

    unsafe {
        display = XOpenDisplay(display_name.as_ptr() as *const c_char);
    }

    if display.is_null() {
        return false;
    }

    unsafe {
        XFixesHideCursor(display, XDefaultRootWindow(display));
        XSync(display, 1);
    }

    true
}

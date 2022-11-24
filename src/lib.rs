use x11::xfixes::XFixesHideCursor;
use x11::xlib::{XDefaultRootWindow, XOpenDisplay, XSync};

pub fn x_hide_cursor(display_number: i8) -> bool {
    let display;

    unsafe {
        display = XOpenDisplay(&display_number);
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

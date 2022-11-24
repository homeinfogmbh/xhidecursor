use std::ffi::{c_char, CString};
use x11::xfixes::XFixesHideCursor;
use x11::xlib::{self, XDefaultRootWindow, XOpenDisplay, XSync};

pub struct Display<'a> {
    display: &'a mut xlib::Display,
}

impl<'a> Display<'a> {
    pub fn open(name: impl Into<String>) -> Option<Self> {
        match CString::new(name.into()) {
            Ok(name) => unsafe { XOpenDisplay(name.as_ptr() as *const c_char).as_mut() }
                .map(|display| Self { display }),
            Err(_) => None,
        }
    }

    pub fn default_root_window(&mut self) -> u64 {
        unsafe { XDefaultRootWindow(self.display) }
    }

    pub fn hide_cursor(&mut self, window: u64) {
        unsafe { XFixesHideCursor(self.display, window) }
    }

    pub fn sync(&mut self, discard: bool) {
        match unsafe { XSync(self.display, discard as i32) } {
            1 => (),
            _ => unreachable!("XSync always returns 1"),
        }
    }
}

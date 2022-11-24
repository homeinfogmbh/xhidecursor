use std::ffi::{c_char, CString};
use x11::xfixes::XFixesHideCursor;
use x11::xlib::{self, XDefaultRootWindow, XOpenDisplay, XSync};

pub struct Display<'a> {
    display: &'a mut xlib::Display,
}

impl<'a> Display<'a> {
    pub fn open(name: Option<impl Into<String>>) -> Option<Self> {
        match name {
            Some(name) => match CString::new(name.into()) {
                Ok(name) => Self::open_raw(name.as_ptr() as *const c_char),
                Err(_) => None,
            },
            None => Self::open_raw(&0),
        }
    }

    fn open_raw(display: *const c_char) -> Option<Self> {
        unsafe { XOpenDisplay(display).as_mut() }.map(|display| Self { display })
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

use std::ffi::CString;

use crate::bindings;

pub struct WindowIcon {
  pub(crate) raw: *mut bindings::rendering_WindowIcon,
}

pub struct WindowIconAsset(*mut bindings::Asset<bindings::rendering_WindowIcon>);

impl WindowIconAsset {
  pub fn new(path: &str) -> Self {
    let path = CString::new(path.as_bytes()).unwrap();

    let raw = unsafe { bindings::create_icon(path.as_ptr()) };

    Self(raw as *mut bindings::Asset<bindings::rendering_WindowIcon>)
  }

  pub fn load(&self) -> WindowIcon {
    let raw = unsafe { bindings::load_icon(self.0 as *mut u8) };

    WindowIcon {
      raw: raw as *mut bindings::rendering_WindowIcon,
    }
  }
}

impl WindowIcon {
  pub fn use_icon(&self) { unsafe { bindings::use_icon(self.raw) }; }
}

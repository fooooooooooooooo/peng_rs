use std::ffi::CString;

use crate::bindings;

pub struct SceneLoader {
  pub(crate) raw: bindings::scene_SceneLoader,
}

impl SceneLoader {
  pub fn new(path: &str) -> Self {
    let path = CString::new(path.as_bytes()).unwrap();

    let raw = unsafe { bindings::load_scene_from_file(path.as_ptr()) };

    Self { raw }
  }
}

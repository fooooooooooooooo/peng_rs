use std::ffi::CString;

use ffi::{
  engine_set_target_frametime, get_window_subsystem, load_icon, load_scene_from_file, on_engine_initialized_once,
  window_set_resolution, window_set_vsync,
};
use libffi::high::Closure1;

use crate::ffi::{create_icon, use_icon};

pub mod ffi;

pub struct PengEngine {
  raw: *mut ffi::PengEngine,
}

impl PengEngine {
  pub fn new() -> Self {
    let raw = unsafe { ffi::create_engine() };

    Self { raw }
  }

  pub fn run(&self) { unsafe { ffi::engine_run(self.raw) } }

  pub fn on_initialized<F: Fn() + Send + Sync + 'static>(&self, f: F) {
    let closure: &'static _ = Box::leak(Box::new(move |_: ()| f()));
    let callback = Closure1::new(closure);
    let &code = callback.code_ptr();
    let ptr: unsafe extern "C" fn() = unsafe { std::mem::transmute(code) };
    std::mem::forget(callback);

    unsafe { on_engine_initialized_once(self.raw, ptr) };
  }

  pub fn set_target_frametime(&self, frametime_ms: f32) {
    unsafe { engine_set_target_frametime(self.raw, frametime_ms) }
  }
}

impl Default for PengEngine {
  fn default() -> Self { Self::new() }
}

pub struct Asset<T> {
  raw: *mut ffi::Asset<T>,
}

pub struct WindowIcon {
  raw: *mut ffi::WindowIcon,
}

impl Asset<ffi::WindowIcon> {
  pub fn new(path: &str) -> Self {
    let path = CString::new(path.as_bytes()).unwrap();

    let raw = unsafe { create_icon(path.as_ptr()) };

    Self { raw }
  }

  pub fn load(&self) -> WindowIcon {
    let raw = unsafe { load_icon(self.raw) };

    WindowIcon { raw }
  }
}

impl WindowIcon {
  pub fn use_icon(&self) { unsafe { use_icon(self.raw) }; }
}

pub struct SceneLoader {
  raw: ffi::SceneLoader,
}

impl SceneLoader {
  pub fn new(path: &str) -> Self {
    let path = CString::new(path.as_bytes()).unwrap();

    let raw = unsafe { load_scene_from_file(path.as_ptr()) };

    Self { raw }
  }
}

pub struct WindowSubsystem {
  raw: *mut ffi::WindowSubsystem,
}

impl WindowSubsystem {
  pub fn get() -> Self {
    let raw = unsafe { get_window_subsystem() };

    Self { raw }
  }

  pub fn set_resolution(&self, resolution: Vector2i) { unsafe { window_set_resolution(self.raw, resolution.raw) }; }

  pub fn set_vsync(&self, vsync: bool) { unsafe { window_set_vsync(self.raw, vsync) }; }
}

pub struct Vector2i {
  raw: ffi::Vector2i,
}

impl Vector2i {
  pub fn new(x: i32, y: i32) -> Self {
    Self {
      raw: { ffi::Vector2i { x, y } },
    }
  }
}

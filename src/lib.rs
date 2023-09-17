use std::ffi::CString;

use libffi::high::Closure1;

pub mod bindings;

pub struct PengEngine {
  raw: *mut bindings::PengEngine,
}

impl PengEngine {
  pub fn new() -> Self {
    let raw = unsafe { bindings::create_engine() };

    Self { raw }
  }

  pub fn run(&self) { unsafe { bindings::engine_run(self.raw) } }

  pub fn on_initialized<F: Fn() + Send + Sync + 'static>(&self, f: F) {
    let closure: &'static _ = Box::leak(Box::new(move |_: ()| f()));
    let callback = Closure1::new(closure);
    let &code = callback.code_ptr();
    let ptr: unsafe extern "C" fn() = unsafe { std::mem::transmute(code) };
    std::mem::forget(callback);

    unsafe { bindings::on_engine_initialized_once(self.raw, Some(ptr)) };
  }

  pub fn set_target_frametime(&self, frametime_ms: f32) {
    unsafe { bindings::engine_set_target_frametime(self.raw, frametime_ms) }
  }
}

impl Default for PengEngine {
  fn default() -> Self { Self::new() }
}

pub struct Asset<T> {
  raw: *mut bindings::Asset<T>,
}

pub struct WindowIcon {
  raw: *mut bindings::rendering_WindowIcon,
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

pub struct SceneLoader {
  raw: bindings::scene_SceneLoader,
}

impl SceneLoader {
  pub fn new(path: &str) -> Self {
    let path = CString::new(path.as_bytes()).unwrap();

    let raw = unsafe { bindings::load_scene_from_file(path.as_ptr()) };

    Self { raw }
  }
}

pub struct WindowSubsystem {
  raw: *mut bindings::rendering_WindowSubsystem,
}

impl WindowSubsystem {
  pub fn get() -> Self {
    let raw = unsafe { bindings::get_window_subsystem() };

    Self { raw }
  }

  pub fn set_resolution(&self, resolution: Vector2i) {
    unsafe { bindings::window_set_resolution(self.raw, resolution.raw) };
  }

  pub fn set_vsync(&self, vsync: bool) { unsafe { bindings::window_set_vsync(self.raw, vsync) }; }
}

pub struct Vector2i {
  raw: bindings::Vector2i,
}

impl Vector2i {
  pub fn new(x: i32, y: i32) -> Self {
    Self {
      raw: { bindings::Vector2i { x, y } },
    }
  }
}

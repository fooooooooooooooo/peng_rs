use std::ffi::{c_char, c_void};

pub type PengEngine = c_void;
/// if you deref this you might die
pub type Asset<T> = *const T;
pub type WindowIcon = c_void;
pub type WindowSubsystem = c_void;

#[repr(C)]
pub struct SceneLoader {
  _0: u8,
}

#[repr(C)]
pub struct Vector2i {
  pub x: i32,
  pub y: i32,
}

#[link(name = "peng_wrapper")]
extern "C" {
  pub fn create_engine() -> *mut PengEngine;
  pub fn engine_run(engine: *mut PengEngine);
  pub fn engine_set_target_frametime(engine: *mut PengEngine, frametime_ms: f32);
  pub fn on_engine_initialized_once(engine: *mut PengEngine, listener: unsafe extern "C" fn());
  pub fn create_icon(path: *const c_char) -> *mut Asset<WindowIcon>;
  pub fn load_icon(icon: *mut Asset<WindowIcon>) -> *mut WindowIcon;
  pub fn use_icon(icon: *mut WindowIcon);
  pub fn load_scene_from_file(path: *const c_char) -> SceneLoader;
  pub fn get_window_subsystem() -> *mut WindowSubsystem;
  pub fn window_set_resolution(window: *mut WindowSubsystem, resolution: Vector2i);
  pub fn window_set_vsync(window: *mut WindowSubsystem, vsync: bool);
  pub fn vector_2i(x: i32, y: i32) -> Vector2i;
}

use crate::bindings;
use crate::utils::closure_to_extern_fn;

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
    let ptr = closure_to_extern_fn(f);

    unsafe { bindings::on_engine_initialized_once(self.raw, Some(ptr)) };
  }

  pub fn set_target_frametime(&self, frametime_ms: f32) {
    unsafe { bindings::engine_set_target_frametime(self.raw, frametime_ms) }
  }
}

impl Default for PengEngine {
  fn default() -> Self { Self::new() }
}

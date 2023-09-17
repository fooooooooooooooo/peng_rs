use crate::{bindings, math::vector2::Vector2i};

pub struct WindowSubsystem {
  pub(crate) raw: *mut bindings::rendering_WindowSubsystem,
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

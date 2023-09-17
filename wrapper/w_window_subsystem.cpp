#include "w_window_subsystem.h"

extern "C" {
  rendering::WindowSubsystem *get_window_subsystem() {
    return &rendering::WindowSubsystem::get();
  }

  void window_set_resolution(
      rendering::WindowSubsystem *window,
      Vector2i resolution) {
    window->set_resolution(math::Vector2i {resolution.x, resolution.y});
  }

  void window_set_vsync(rendering::WindowSubsystem *window, bool vsync) {
    window->set_vsync(vsync);
  }
}

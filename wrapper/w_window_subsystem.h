#pragma once

#include "rendering/window_subsystem.h"
#include "w_vector2.h"

extern "C" {
  rendering::WindowSubsystem *get_window_subsystem();

  void window_set_resolution(rendering::WindowSubsystem *window, Vector2i resolution);

  void window_set_vsync(rendering::WindowSubsystem *window, bool vsync);
}

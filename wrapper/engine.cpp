#include "engine.h"

#pragma clang diagnostic push
#pragma ide diagnostic ignored "OCUnusedGlobalDeclarationInspection"

extern "C" {
PengEngine *create_engine() {
  return &PengEngine::get();
}

void engine_run(PengEngine *engine) {
  engine->run();
}

void engine_set_target_frametime(PengEngine *engine, float frametime_ms) {
  engine->set_target_frametime(frametime_ms);
}

void on_engine_initialized_once(PengEngine *engine, void (listener)(void)) {
  engine->on_engine_initialized().subscribe_once(listener);
}

Asset<rendering::WindowIcon> *create_icon(const char *path) {
  return new Asset<rendering::WindowIcon>(path);
}

const rendering::WindowIcon *load_icon(Asset<rendering::WindowIcon> *icon) {
  return icon->load().get();
}

void use_icon(rendering::WindowIcon *icon) {
  icon->use();
}

scene::SceneLoader load_scene_from_file(const char *path) {
  scene::SceneLoader scene_loader;

  scene_loader.load_from_file(path);

  return scene_loader;
}

rendering::WindowSubsystem *get_window_subsystem() {
  return &rendering::WindowSubsystem::get();
}

void window_set_resolution(rendering::WindowSubsystem *window, math::Vector2i resolution) {
  window->set_resolution(resolution);
}

void window_set_vsync(rendering::WindowSubsystem *window, bool vsync) {
  window->set_vsync(vsync);
}

math::Vector2i vector_2i(int x, int y) {
  return { x, y };
}
}

#pragma clang diagnostic pop

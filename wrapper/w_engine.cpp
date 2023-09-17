#include "w_engine.h"

extern "C" {
  PengEngine *create_engine() { return &PengEngine::get(); }

  void engine_run(PengEngine *engine) { engine->run(); }

  void engine_set_target_frametime(PengEngine *engine, float frametime_ms) {
    engine->set_target_frametime(frametime_ms);
  }

  void on_engine_initialized_once(PengEngine *engine, void(listener)(void)) {
    engine->on_engine_initialized().subscribe_once(listener);
  }

  Asset<rendering::WindowIcon> *create_icon(const char *path) {
    return new Asset<rendering::WindowIcon>(path);
  }

  const rendering::WindowIcon *load_icon(Asset<rendering::WindowIcon> *icon) {
    return icon->load().get();
  }

  void use_icon(rendering::WindowIcon *icon) { icon->use(); }

  scene::SceneLoader load_scene_from_file(const char *path) {
    scene::SceneLoader scene_loader;

    scene_loader.load_from_file(path);

    return scene_loader;
  }
}

use peng_rs::ffi::{self};
use peng_rs::{Asset, PengEngine, SceneLoader, Vector2i, WindowSubsystem};

fn main() {
  let engine = PengEngine::new();

  engine.on_initialized(|| {
    let icon = Asset::<ffi::WindowIcon>::new("resources/textures/core/peng_engine_64.asset");
    icon.load().use_icon();

    let scene_loader = SceneLoader::new("resources/scenes/demo/pong.json");
    Box::leak(Box::new(scene_loader));
  });

  let window_subsystem = WindowSubsystem::get();

  window_subsystem.set_resolution(Vector2i::new(1280, 720));
  window_subsystem.set_vsync(false);

  engine.set_target_frametime(0.0);
  engine.run()
}

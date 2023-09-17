use peng_rs::core::engine::PengEngine;
use peng_rs::math::vector2::Vector2i;
use peng_rs::rendering::window_icon::WindowIconAsset;
use peng_rs::rendering::window_subsystem::WindowSubsystem;
use peng_rs::scene::SceneLoader;

fn main() {
  let engine = PengEngine::new();

  engine.on_initialized(|| {
    let icon = WindowIconAsset::new("resources/textures/core/peng_engine_64.asset");
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

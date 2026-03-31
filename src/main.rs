use macroquad::prelude::*;
use rust_physics_engine::graphics::MacroquadRenderer;
use rust_physics_engine::physics::rigidbody::Rigidbody;
use rust_physics_engine::physics::World;
use rust_physics_engine::shape::line::Line;

#[macroquad::main("Test Render")]
async fn main() {
  let mut world: World = World::new();
  let mut macroquad_renderer = MacroquadRenderer::new();
  world.add_object(Rigidbody::new(Line::new(20)).unwrap());

  loop {
    clear_background(WHITE);
    world.render(&mut macroquad_renderer);
    next_frame().await
  }
}
use macroquad::prelude::*;
use rust_physics_engine::physics::rigidbody::Rigidbody;
use rust_physics_engine::physics::World;
use rust_physics_engine::shape::line::Line;

#[macroquad::main("Test Render")]
async fn main() {
  loop {
    clear_background(WHITE);
    let mut world: World = World::new();
    world.add_object(Rigidbody::new(Line::new(20)).unwrap());
    world.render();

    next_frame().await
  }
}
mod graphics;
mod physics;

use macroquad::prelude::*;
use crate::graphics::graphics_manager::GraphicsManager;
use crate::graphics::square::Square;
use physics::point_2d::Point2D;

#[macroquad::main("Physics Engine")]
async fn main() {
  let point_a: Point2D = Point2D::new(1, 2);
  print!("{}", point_a);

  let mut graphics_manager: GraphicsManager = GraphicsManager::new();
  graphics_manager.add_shape(Square::new());
  loop {
    clear_background(WHITE);
    graphics_manager.render();
    next_frame().await
  }
}

mod Graphics;
mod Physics;

use macroquad::prelude::*;
use crate::Graphics::GraphicsManager::GraphicsManager;
use crate::Graphics::Square::Square;
use crate::Physics::Point2D::Point2D;

#[macroquad::main("Physics Engine")]
async fn main() {
  print!("{}", Point2D::new(1.0, 2.0));

  let mut graphics_manager: GraphicsManager = GraphicsManager::new();
  graphics_manager.add_shape(Square::new());
  loop {
    clear_background(WHITE);
    graphics_manager.render();
    next_frame().await
  }
}

mod Graphics;

use macroquad::prelude::*;
use crate::Graphics::GraphicsManager::GraphicsManager;
use crate::Graphics::Square::Square;

#[macroquad::main("Physics Engine")]
async fn main() {
  let mut graphics_manager: GraphicsManager = GraphicsManager::new();
  graphics_manager.add_shape(Square::new());
  loop {
    clear_background(WHITE);
    graphics_manager.render();
    next_frame().await
  }
}

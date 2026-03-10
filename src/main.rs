mod graphics;
mod physics;

use crate::graphics::line::Line;
use macroquad::prelude::*;
use crate::graphics::canvas::Canvas;
use crate::physics::vector2::Vector2;

#[macroquad::main("Physics Engine")]
async fn main() {
  let mut graphics_manager: Canvas = Canvas::new();
  graphics_manager.add_shape(Line::new(Vector2::new(1, 1), Vector2::new(20, 20)));
  loop {
    clear_background(WHITE);
    graphics_manager.render();
    next_frame().await
  }
}

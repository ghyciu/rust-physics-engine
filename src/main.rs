extern crate core;

mod graphics;
mod physics;

use crate::graphics::line::Line;
use crate::graphics::circle::Circle;
use macroquad::prelude::*;
use crate::graphics::canvas::Canvas;
use crate::physics::vector2::Vector2;
use crate::physics::math::Math;

#[macroquad::main("Physics Engine")]
async fn main() {
  let mut canvas: Canvas = Canvas::new();
  canvas.add_shape(Line::new(Vector2::ZERO, Vector2::new(20, 20)));
  canvas.add_shape(Circle::new(Vector2::ZERO, 20_f32));
  print!("{}", Math::length(Vector2::new(3, 4)));
  loop {
    clear_background(WHITE);
    canvas.render();
    next_frame().await
  }
}

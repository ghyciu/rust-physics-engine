use crate::Graphics::Shape::Shape;
use macroquad::prelude::*;
pub struct Square;

impl Shape for Square {
  fn render(&self) {
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
  }
}

impl Square {
  pub fn new() -> Square {
    Square
  }
}
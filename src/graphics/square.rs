use macroquad::prelude::*;
use crate::graphics::shape::Render;

pub struct Square;

impl Square {
  pub fn new() -> Square {
    Square
  }
}

impl Render for Square {
  fn render(&self) {
    draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
  }
}


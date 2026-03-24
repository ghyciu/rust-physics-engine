use std::fmt;
use macroquad::color::BLACK;
use macroquad::prelude::draw_line;
use macroquad::window::screen_height;

use crate::graphics::Renderable;
use crate::math::length::{Length, LengthError};
use crate::math::Vector2;
use crate::shape::line::LineResult;

#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/line.md"))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line(Length);

impl Line {
  /// Creates a new [`Line`] object. Returns a [`LineResult`](super::LineResult).
  pub fn new<T>(length: T) -> LineResult
  where T: TryInto<Length, Error = LengthError> {
    Ok(Line{ 0: length.try_into()?})
  }

  /// Gets the `length` of the [`Line`]. Used only when printing to console.
  fn get_length(&self) -> f32 {
    self.0.get()
  }
}

impl fmt::Display for Line {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Line({})", self.get_length())
  }
}

impl Renderable for Line {
  fn render(&self, position: Vector2) {
    let position_x: f32 = position.get_x();
    let position_y: f32 = position.get_y();
    let vector_x: f32 = self.get_length();
    let vector_y: f32 = self.get_length();
    let screen_height: f32 = screen_height();
    draw_line(position_x, screen_height - position_y, position_x + vector_x, screen_height - position_y - vector_y, 1.0f32, BLACK);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    assert!(Line::new(10.0).is_ok());
    assert!(Line::new(10).is_ok());
  }

  #[test]
  fn get_radius() {
    assert_eq!(Line::new(10).unwrap().get_length(), 10.0_f32);
  }

  #[test]
  fn print() {
    assert_eq!(Line::new(10).unwrap().to_string(), "Line(10)");
  }
}
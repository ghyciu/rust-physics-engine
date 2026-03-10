use macroquad::color::BLACK;
use macroquad::shapes::draw_line;
use macroquad::window::screen_height;
use super::renderable::Renderable;
use crate::physics::vector2::Vector2;

/// Represents a line of length `vector` starting from the `origin`.
pub struct Line {
  origin: Vector2,
  vector: Vector2
}

impl Line {
  /// Creates a new `Line`.
  pub fn new(origin: Vector2, vector: Vector2) -> Line {
    Line { origin, vector }
  }
}

/// Draws a line of length `vector` starting from the `origin`.
impl Renderable for Line {
  fn render(&self) {
    let origin_x: f32 = self.origin.get_x();
    let origin_y: f32 = self.origin.get_y();
    let vector_x: f32 = self.vector.get_x();
    let vector_y: f32 = self.vector.get_y();
    let screen_height: f32 = screen_height();
    draw_line(origin_x, screen_height - origin_y, origin_x + vector_x, screen_height - origin_y - vector_y, 1.0f32, BLACK);
  }
}
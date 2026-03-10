use macroquad::color::BLACK;
use macroquad::window::screen_height;
use macroquad::shapes::draw_circle_lines;
use crate::graphics::renderable::Renderable;
use crate::physics::scalar::Scalar;
use crate::physics::vector2::Vector2;

pub struct Circle {
  position: Vector2,
  radius: f32,
}

impl Circle {
  /// Creates a new `Circle` object with center at position `Vector2` with radius `Scalar`.
  pub fn new<T: Scalar>(position: Vector2, radius: T) -> Circle {
    Circle { position, radius: radius.to_scalar()}
  }
}

impl Renderable for Circle {
  fn render(&self) {
    draw_circle_lines(self.position.get_x(), screen_height() - self.position.get_y(), self.radius, 1.0f32, BLACK);
  }
}
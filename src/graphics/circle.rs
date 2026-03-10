use macroquad::color::BLACK;
use macroquad::shapes::draw_circle;
use crate::graphics::renderable::Renderable;
use crate::physics::vector2::Vector2;

pub struct Circle {
  origin: Vector2,
  radius: f32,
}

impl Circle {
  pub fn new(origin: Vector2, radius: f32) -> Circle {
    Circle { origin, radius}
  }
}

impl Renderable for Circle {
  fn render(&self) {
    draw_circle(self.origin.get_x(), macroquad::prelude::screen_height() - self.origin.get_y(), self.radius, BLACK);
  }
}
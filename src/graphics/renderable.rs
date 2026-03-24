use crate::math::Vector2;

pub trait Renderable {
  fn render(&self, position: Vector2);
}
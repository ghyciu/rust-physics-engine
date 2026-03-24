use std::fmt;
use crate::graphics::Renderable;
use crate::math::Vector2;
use crate::shape::line::Line;
use super::circle::Circle;

/// A graphical representation of an object. Each variant of [`Shape`] can contain different fields. Required by
/// [`RigidBody`](crate::physics::rigidbody::Rigidbody).
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Shape {
  Line(Line),
  Circle(Circle)
}

impl fmt::Display for Shape {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Shape::Line(line) => write!(f, "{}", line),
      Shape::Circle(circle) => write!(f, "{}", circle),
    }
  }
}

impl Renderable for Shape {
  fn render(&self, position: Vector2) {
    match self {
      Shape::Line(line) => line.render(position),
      _ => unimplemented!()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn print_line() {
    assert_eq!(Shape::Line(Line::new(10.0).unwrap()).to_string(), "Line(10)");
  }

  #[test]
  fn print_circle() {
    assert_eq!(Shape::Circle(Circle::new(10.0).unwrap()).to_string(), "Circle(10)");
  }
}
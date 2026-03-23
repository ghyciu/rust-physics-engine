use std::fmt;
use super::circle::Circle;

/// A graphical representation of an object. Each variant of [`Shape`] can contain different fields. Required by
/// [`RigidBody`](crate::physics::RigidBody).
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Shape {
  Circle(Circle)
}

impl fmt::Display for Shape {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Shape::Circle(circle) => write!(f, "{}", circle),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn print_circle() {
    assert_eq!(Shape::Circle(Circle::new(10.0).unwrap()).to_string(), "Circle(10)");
  }
}
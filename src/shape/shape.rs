use std::fmt;
use super::circle::Circle;

/// A graphical representation of an object. Each variant of [`Shape`] can contain different fields. Required by
/// [`RigidBody`](crate::physics::RigidBody).
#[derive(Debug, Copy, Clone)]
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
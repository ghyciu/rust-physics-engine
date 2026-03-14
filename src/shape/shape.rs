use std::fmt;
use super::circle::Circle;

/// Physical characteristics of a [`RigidBody`](RigidBody). Each variant of [`Shape`] can contain different fields.
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
use std::fmt;
use super::circle::{Circle, CircleResult};

/// A `Shape` is any object implemented by `RigidBody` which contains a specific set of values for
/// how they will be rendered on the screen. Different shapes will have different `Shape` fields.
pub enum Shape {
  Circle(Circle)
}

impl Copy for Shape {}
impl Clone for Shape {
  fn clone(&self) -> Shape {
    match self {
      Shape::Circle(circle) => Shape::Circle(circle.clone()),
    }
  }
}

/// Prints the implementing `Shape`. When a `Shape` is printed, it calls the corresponding `fmt()` method of
/// the implementing `Shape`.
impl fmt::Display for Shape {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Shape::Circle(circle) => write!(f, "{}", circle),
    }
  }
}
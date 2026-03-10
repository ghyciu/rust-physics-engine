use std::fmt;
use super::scalar::Scalar;

/// Represents a circle with radius `radius`.
/// # Construction
/// ```
/// let circle = Circle::new(5)
/// ```
pub struct Circle {
  radius: f32,
}

impl Circle {
  /// Creates a new `Circle` with radius `radius`.
  pub fn new<T: Scalar>(radius: T) -> Self {
    Circle {radius: radius.to_scalar()}
  }

  /// Private getter to get `circle.radius`. Used only when printing to console.
  fn get_radius(&self) -> f32 {
    self.radius
  }
}

impl Copy for Circle {}
impl Clone for Circle {
  fn clone(&self) -> Circle {
    *self
  }
}

/// Prints the `Circle` object represented as `Circle(radius)` in text.
impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle({})", self.get_radius())
  }
}
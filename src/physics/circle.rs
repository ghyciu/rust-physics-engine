use std::fmt;
use crate::physics::circle_radius::CircleRadius;
use super::scalar::Scalar;

/// Represents a circle with radius `radius`.
/// # Construction
/// ```
/// let circle = Circle::new(5)
/// ```
pub struct Circle {
  radius: CircleRadius,
}

pub type CircleResult = Result<Circle, &'static str>;

impl Circle {
  /// Creates a new `Circle` with radius `Scalar`. Returns a `CircleResult` which may
  /// result in an `Error`.
  pub fn new<T: Scalar>(radius: T) -> CircleResult {
    let radius: CircleRadius = CircleRadius::new(radius.to_scalar())?;
    Ok(Circle{ radius })
  }

  /// Private getter to get `circle.radius`. Used only when printing to console.
  fn get_radius(&self) -> f32 {
    self.radius.get()
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
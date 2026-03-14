use std::fmt;
use crate::math::Scalar;
use super::{CircleRadius, CircleError};

/// Circle with radius of length `radius`.
/// # Construction
/// ```
/// let circle = Circle::new(5);
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Circle {
  radius: CircleRadius,
}

/// [`Result`] returned when creating a new [`Circle`]. May return a [`CircleError`]
/// if [`CircleRadiusResult`](super::CircleRadiusResult) is invalid.
pub type CircleResult = Result<Circle, CircleError>;

impl Circle {
  /// Creates a new [`Circle`] object. Returns a [`CircleResult`](super::CircleResult).
  pub fn new<T: Scalar>(radius: T) -> CircleResult {
    let radius: CircleRadius = CircleRadius::new(radius.to_scalar())?;
    Ok(Circle{ radius })
  }

  /// Gets the `radius` of the [`Circle`]. Used only when printing to console.
  fn get_radius(&self) -> f32 {
    self.radius.get()
  }
}

impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle({})", self.get_radius())
  }
}
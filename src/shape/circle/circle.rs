use std::fmt;
use crate::math::Scalar;
use super::{CircleResult, CircleRadius};

/// A geometric [circle](https://en.wikipedia.org/wiki/Circle) with radius of length `radius`.
#[derive(Debug, Copy, Clone)]
pub struct Circle {
  radius: CircleRadius,
}

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
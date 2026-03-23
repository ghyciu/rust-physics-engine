use std::fmt;
use crate::math::length::{Length, LengthError};
use super::{CircleResult};

#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/circle.md"))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Circle(Length);

impl Circle {
  /// Creates a new [`Circle`] object. Returns a [`CircleResult`](super::CircleResult).
  pub fn new<T>(radius: T) -> CircleResult
  where T: TryInto<Length, Error = LengthError> {
    Ok(Circle{ 0: radius.try_into()?})
  }

  /// Gets the `radius` of the [`Circle`]. Used only when printing to console.
  fn get_radius(&self) -> f32 {
    self.0.get()
  }
}

impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle({})", self.get_radius())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    assert!(Circle::new(10.0).is_ok());
  }

  #[test]
  fn get_radius() {
    assert_eq!(Circle::new(10.0).unwrap().get_radius(), 10.0_f32);
  }

  #[test]
  fn print() {
    assert_eq!(Circle::new(10.0).unwrap().to_string(), "Circle(10)");
  }
}
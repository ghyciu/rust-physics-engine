use std::fmt;
use crate::math::length::{Length, LengthError};
use crate::shape::line::LineResult;

#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/line.md"))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Line(Length);

impl Line {
  pub fn new<L>(length: L) -> LineResult
  where L: TryInto<Length, Error = LengthError>, {
    Ok(Line{ 0: length.try_into()?})
  }

  /// Gets the `length` of the [`Line`]. Used only when printing to console.
  fn get_length(&self) -> f32 {
    self.0.get()
  }
}

impl fmt::Display for Line {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Line({})", self.get_length())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    assert!(Line::new(10.0).is_ok());
    assert!(Line::new(10).is_ok());
  }

  #[test]
  fn get_radius() {
    assert_eq!(Line::new(10.0).unwrap().get_length(), 10.0_f32);
  }

  #[test]
  fn print() {
    assert_eq!(Line::new(10.0).unwrap().to_string(), "Line(10)");
  }
}
use super::{LengthError, LengthResult};
use crate::math::Scalar;
use std::convert::TryFrom;

#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/length.md"))]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Length(f32);

impl Length {
  /// Creates a new [`Length`] object. Returns a [`LengthResult`].
  pub fn new<T: Scalar>(value: T) -> LengthResult {
    let value: f32 = value.to_scalar();
    if value <= 0_f32 {
      return Err(LengthError::NonPositiveError);
    }
    Ok(Length(value))
  }

  /// Returns the value of [`Length`].
  pub fn get(&self) -> f32 {
    self.0
  }
}

/// Implementation for converting an `i32` type to a [`Length`] object.
impl TryFrom<i32> for Length {
  type Error = LengthError;
  fn try_from(value: i32) -> Result<Self, Self::Error> {
    Length::new(value)
  }
}

/// Implementation for converting an `f32` type to a [`Length`] object.
impl TryFrom<f32> for Length {
  type Error = LengthError;
  fn try_from(value: f32) -> Result<Self, Self::Error> {
    Length::new(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    assert_eq!(Length::new(10.0_f32), Ok(Length(10.0)));
    assert_eq!(Length::new(-10.0_f32), Err(LengthError::NonPositiveError));
  }

  #[test]
  fn get() {
    assert_eq!(Length::new(10.0_f32).unwrap().get(), 10.0_f32);
  }
}
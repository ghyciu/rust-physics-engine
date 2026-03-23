use crate::math::length::LengthError;

/// [`Err`] returned by [`CircleResult`](super::CircleResult). A type of [`ShapeError`](crate::shape::ShapeError).
#[derive(Debug, PartialEq)]
pub enum CircleError {
  /// The radius of the circle is invalid.
  LengthError(LengthError)
}

impl From<LengthError> for CircleError {
  fn from(value: LengthError) -> Self {
    CircleError::LengthError(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn from_circle_radius_error() {
    let circle_error: CircleError = LengthError::NonPositiveError.into();
    assert_eq!(CircleError::LengthError(LengthError::NonPositiveError), circle_error);
  }
}
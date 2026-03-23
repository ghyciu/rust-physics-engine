use crate::shape::ShapeError;

#[derive(Debug, PartialEq)]
/// [`Err`] returned by [`RigidbodyResult`](super::RigidbodyResult).
pub enum RigidbodyError {
  ShapeError(ShapeError)
}

impl From<ShapeError> for RigidbodyError {
  fn from(value: ShapeError) -> Self {
    RigidbodyError::ShapeError(value)
  }
}

#[cfg(test)]
mod tests {
  use crate::math::length::LengthError;
  use crate::shape::circle::{CircleError};
  use crate::shape::ShapeError;
  use super::RigidbodyError;

  #[test]
  fn from_shape_error() {
    let rigidbody_error: RigidbodyError = ShapeError::CircleError(CircleError::LengthError(LengthError::NonPositiveError)).into();
    assert_eq!(RigidbodyError::ShapeError(ShapeError::CircleError(CircleError::LengthError(LengthError::NonPositiveError))), rigidbody_error);
  }
}
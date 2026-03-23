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
  use crate::shape::circle::{CircleError, CircleRadiusError};
  use crate::shape::ShapeError;
  use super::RigidbodyError;

  #[test]
  fn from_shape_error() {
    let rigidbody_error: RigidbodyError = ShapeError::CircleError(CircleError::CircleRadiusError(CircleRadiusError::NonPositiveError)).into();
    assert_eq!(RigidbodyError::ShapeError(ShapeError::CircleError(CircleError::CircleRadiusError(CircleRadiusError::NonPositiveError))), rigidbody_error);
  }
}
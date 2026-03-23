use crate::shape::ShapeError;

#[derive(Debug, PartialEq)]
/// [`Err`] returned by [`RigidBodyResult`].
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
    let rigid_body_error: RigidbodyError = ShapeError::CircleError(CircleError::CircleRadiusError(CircleRadiusError::NonPositiveError)).into();
    assert_eq!(RigidbodyError::ShapeError(ShapeError::CircleError(CircleError::CircleRadiusError(CircleRadiusError::NonPositiveError))), rigid_body_error);
  }
}
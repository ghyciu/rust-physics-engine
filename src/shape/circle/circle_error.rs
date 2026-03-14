use super::CircleRadiusError;

/// [`Err`] returned by [`CircleResult`](super::CircleResult). A variant of [`ShapeError`](crate::physics::shape::ShapeError).
#[derive(Debug)]
pub enum CircleError {
  /// The `radius` is invalid.
  CircleRadiusError(CircleRadiusError)
}

impl From<CircleRadiusError> for CircleError {
  fn from(value: CircleRadiusError) -> Self {
    CircleError::CircleRadiusError(value)
  }
}
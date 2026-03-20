use super::CircleRadiusError;

/// [`Err`] returned by [`CircleResult`](super::CircleResult). A variant of [`ShapeError`](crate::shape::ShapeError).
#[derive(Debug, PartialEq)]
pub enum CircleError {
  /// The `radius` is invalid.
  CircleRadiusError(CircleRadiusError)
}

impl From<CircleRadiusError> for CircleError {
  fn from(value: CircleRadiusError) -> Self {
    CircleError::CircleRadiusError(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn from_circle_radius_error() {
    let circle_error: CircleError = CircleRadiusError::NonPositiveError.into();
    assert_eq!(CircleError::CircleRadiusError(CircleRadiusError::NonPositiveError), circle_error);
  }
}
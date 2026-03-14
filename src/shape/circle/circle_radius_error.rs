use std::fmt;

/// [`Err`] returned by [`CircleRadiusResult`](super::CircleRadiusResult). A variant of
/// [`CircleError`](super::CircleError).
#[derive(Debug)]
pub enum CircleRadiusError {
  /// [`CircleRadius`](super::CircleRadius) is not positive.
  NonPositiveError,
}

impl fmt::Display for CircleRadiusError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CircleRadiusError::NonPositiveError => write!(f, "Radius cannot be negative"),
    }
  }
}

impl std::error::Error for CircleRadiusError{}
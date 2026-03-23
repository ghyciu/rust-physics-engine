use std::fmt;

/// [`Err`] returned by [`CircleRadiusResult`](super::CircleRadiusResult). A variant of
/// [`CircleError`](super::CircleError).
#[derive(Debug, PartialEq)]
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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn print() {
    assert_eq!(CircleRadiusError::NonPositiveError.to_string(), "Radius cannot be negative");
  }
}
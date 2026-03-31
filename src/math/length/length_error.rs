use std::fmt;

/// [`Err`] returned by [`Length`](super::Length). A type of [`ShapeError`](crate::shape::ShapeError).
#[derive(Debug, PartialEq)]
pub enum LengthError {
  /// [`Length`](super::Length) is not positive.
  NonPositiveError,
}

impl fmt::Display for LengthError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      LengthError::NonPositiveError => write!(f, "Length must be positive"),
    }
  }
}

impl std::error::Error for LengthError{}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn print() {
    assert_eq!(LengthError::NonPositiveError.to_string(), "Length must be positive");
  }
}
use crate::math::length::LengthError;

/// [`Err`] returned by [`LineResult`](super::LineResult). A type of [`ShapeError`](crate::shape::ShapeError).
#[derive(Debug, PartialEq)]
pub enum LineError {
  /// The `radius` is invalid.
  LengthError(LengthError)
}

impl From<LengthError> for LineError {
  fn from(value: LengthError) -> Self {
    LineError::LengthError(value)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn from_length_error() {
    let line_error: LineError = LengthError::NonPositiveError.into();
    assert_eq!(LineError::LengthError(LengthError::NonPositiveError), line_error);
  }
}
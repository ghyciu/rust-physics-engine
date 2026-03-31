use crate::shape::{Shape, ShapeError, ShapeResult, ToShapeResult};
use super::{Line, LineError};

/// [`LineResult`] returned when creating a new [`Line`]. Returns a [`LineError`]
/// if [`LengthResult`](crate::math::length::LengthResult) is invalid.
pub type LineResult = Result<Line, LineError>;

/// Implementation for the [`ToShapeResult`] trait for [`LineResult`].
impl ToShapeResult for LineResult {
  /// Converts the [`LineResult`] to a [`ShapeResult`].
  fn to_shape_result(self) -> ShapeResult {
    self.map(Shape::Line).map_err(ShapeError::LineError)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn to_shape_result() {
    let shape: ShapeResult = Line::new(10).to_shape_result();
    assert_eq!(Shape::Line(Line::new(10).unwrap()), shape.unwrap());
  }
}
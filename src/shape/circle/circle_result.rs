use crate::shape::{Shape, ShapeError, ShapeResult, ToShapeResult};
use super::{Circle, CircleError};

/// [`Result`] returned when creating a new [`Circle`]. May return a [`CircleError`]
/// if [`LengthResult`](crate::math::length::LengthResult) is invalid.
pub type CircleResult = Result<Circle, CircleError>;

/// Implementation for the [`ToShapeResult`] trait for [`CircleResult`].
impl ToShapeResult for CircleResult {
  /// Converts the [`CircleResult`] to a [`ShapeResult`].
  fn to_shape_result(self) -> ShapeResult {
    self.map(Shape::Circle).map_err(ShapeError::CircleError)
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn to_shape_result() {
    let shape: ShapeResult = Circle::new(10).to_shape_result();
    assert_eq!(Shape::Circle(Circle::new(10).unwrap()), shape.unwrap());
  }
}
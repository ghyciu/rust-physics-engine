use crate::shape::circle::{CircleResult};
use super::{Shape, ShapeError};

/// [`Result`] whose [`Ok`] variant returns a type of [`Shape`]. May return a [`ShapeError`] if the variant `Shape` is invalid.
///
/// **Variants:** [`CircleResult`]
pub type ShapeResult = Result<Shape, ShapeError>;

/// Trait for any [`ShapeResult`] variant to convert itself to a `ShapeResult`.
/// Each `ShapeResult` variant must implement this trait.
pub trait ToShapeResult {
  /// Converts the [`ShapeResult`] variant to a `ShapeResult`.
  fn to_shape_result(self) -> ShapeResult;
}

/// Implementation for the [`ToShapeResult`] trait for [`CircleResult`].
impl ToShapeResult for CircleResult {
  /// Converts the [`CircleResult`] to a [`ShapeResult`].
  fn to_shape_result(self) -> ShapeResult {
    self.map(Shape::Circle).map_err(ShapeError::CircleError)
  }
}

#[cfg(test)]
mod tests {
  use crate::shape::circle::Circle;
  use super::*;
  #[test]
  fn to_shape_result() {
    let shape: ShapeResult = Circle::new(10).to_shape_result();
    assert_eq!(Shape::Circle(Circle::new(10).unwrap()), shape.unwrap());
  }
}
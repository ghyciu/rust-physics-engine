use crate::shape::circle::{CircleError, CircleResult};
use crate::shape::Shape;

/// [`Result`] whose [`Ok`] variant returns a type of [`Shape`]. May return a [`ShapeError`] if the variant `Shape` is invalid.
///
/// **Variants:** [`CircleResult`]
pub type ShapeResult = Result<Shape, ShapeError>;

/// [`Err`] returned by [`ShapeResult`].
#[derive(Debug)]
pub enum ShapeError {
  CircleError(CircleError)
}

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
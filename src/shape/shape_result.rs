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
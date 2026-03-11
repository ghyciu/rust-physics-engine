use std::fmt;
use super::circle::{Circle, CircleResult};

/// Describes the physical characteristics of a [`RigidBody`]. Each variant of [`Shape`] can contain different fields.
pub enum Shape {
  Circle(Circle)
}

impl Copy for Shape {}
impl Clone for Shape {
  fn clone(&self) -> Shape {
    match self {
      Shape::Circle(circle) => Shape::Circle(circle.clone()),
    }
  }
}

impl fmt::Display for Shape {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Shape::Circle(circle) => write!(f, "{}", circle),
    }
  }
}

/// [`Result`] object generated whose [`Ok`] variant returns a type
/// of [`Shape`]. Called as a parameter when attempting to initialize a [`RigidBody`].
///
/// **Variants:** [`CircleResult`]
pub type ShapeResult = Result<Shape, &'static str>;

/// Ability for any [`ShapeResult`] variant to convert itself to a `ShapeResult`.
/// Each `ShapeResult` variant must implement this trait.
pub trait ToShapeResult {
  /// Converts the [`ShapeResult`] variant to a `ShapeResult`.
  fn to_shape_result(&self) -> ShapeResult;
}

/// Implementation for the [`ToShapeResult`] trait for [`CircleResult`].
impl ToShapeResult for CircleResult {
  /// Converts the [`CircleResult`] to a [`ShapeResult`].
  fn to_shape_result(&self) -> ShapeResult {
    self.map(Shape::Circle)
  }
}
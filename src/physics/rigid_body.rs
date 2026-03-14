use std::fmt;
use crate::shape::{Shape, ShapeError, ShapeResult, ToShapeResult};
use super::body::Body;

/// Any object with physics and characteristics.
#[derive(Debug)]
pub struct RigidBody {
  shape: Shape,
  body: Body,
}

/// [`Result`] returned when creating a new [`RigidBody`]. May return an [`RigidBodyError`] if
/// the [`ShapeResult`] is invalid.
type RigidBodyResult = Result<RigidBody, RigidBodyError>;

impl RigidBody {
  /// Create a new [`RigidBody`]. Returns a [`RigidBodyResult`].
  pub fn new<T: ToShapeResult>(shape_result: T) -> RigidBodyResult {
    let shape_result: ShapeResult = shape_result.to_shape_result();
    let shape: Shape = shape_result?;
    Ok(RigidBody{shape, body: Body::ZERO})
  }

  /// Gets the `shape`.
  fn get_shape(&self) -> Shape {
    self.shape
  }

  /// Gets the `body`.
  fn get_body(&self) -> Body {
    self.body
  }
}

impl fmt::Display for RigidBody {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "RigidBody({} at {})", self.get_shape(), self.get_body())
  }
}

#[derive(Debug)]
/// [`Err`] returned by [`RigidBodyResult`].
pub enum RigidBodyError {
  ShapeError(ShapeError)
}

impl From<ShapeError> for RigidBodyError {
  fn from(value: ShapeError) -> Self {
    RigidBodyError::ShapeError(value)
  }
}
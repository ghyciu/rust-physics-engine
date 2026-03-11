use std::fmt;
use crate::physics::shape::{Shape, ShapeResult, ToShapeResult};
use super::body::Body;

/// Describes any object with physics and characteristics.
pub struct RigidBody {
  shape: Shape,
  body: Body,
}

/// [`Result`] object returned when creating a new [`RigidBody`]. May return an [`Err`] if
/// the [`ShapeResult`] is invalid.
type RigidBodyResult = Result<RigidBody, &'static str>;

impl RigidBody {
  /// Create a new [`RigidBody`]. Returns a [`RigidBodyResult`].
  pub fn new<T: ToShapeResult>(shape_result: T) -> RigidBodyResult {
    let shape_result: ShapeResult = shape_result.to_shape_result();
    let shape: Shape = shape_result?;
    Ok(RigidBody{shape, body: Body::ZERO})
  }

  /// Get the `shape` of the [`RigidBody`]. Returns a [`Shape`].
  fn get_shape(&self) -> Shape {
    self.shape
  }

  /// Get the `body` of the [`RigidBody`]. Returns a [`Body`].
  fn get_body(&self) -> Body {
    self.body
  }
}

impl fmt::Display for RigidBody {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "RigidBody({} at {})", self.get_shape(), self.get_body())
  }
}
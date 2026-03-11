use std::fmt;
use crate::physics::circle::{Circle};
use crate::physics::shape::{Shape};
use crate::physics::shape_result::{ToShapeResult, ShapeResult};
use super::body::Body;

/// A `RigidBody` is any component with a general set of values in `Body` and a specific set of values
/// `Shape` on how they will be rendered on the screen.
pub struct RigidBody {
  shape: Shape,
  body: Body,
}

type RigidBodyResult = Result<RigidBody, &'static str>;

impl RigidBody {
  pub fn new<T: ToShapeResult>(shape_result: T) -> RigidBodyResult {
    let shape_result: ShapeResult = shape_result.to_shape_result();
    let shape: Shape = shape_result?;
    Ok(RigidBody{shape, body: Body::ZERO})
  }

  fn get_shape(&self) -> Shape {
    self.shape
  }

  fn get_body(&self) -> Body {
    self.body
  }
}

impl fmt::Display for RigidBody {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "RigidBody({} at {})", self.get_shape(), self.get_body())
  }
}
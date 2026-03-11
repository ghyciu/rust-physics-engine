use std::fmt;
use crate::physics::circle::CircleResult;
use crate::physics::shape_result::ShapeResult;
use super::body::Body;
use super::shape::Shape;

/// A `RigidBody` is any component with a general set of values in `Body` and a specific set of values
/// `Shape` on how they will be rendered on the screen.
pub struct RigidBody {
  shape: ShapeResult,
  body: Body,
}

type RigidBodyResult = Result<RigidBody, &'static str>;

impl RigidBody {
  pub fn new<T: Into<ShapeResult>>(shape_result: T) -> RigidBody {
    let shape_result: ShapeResult = shape_result.into();
    RigidBody {shape: shape_result, body: Body::ZERO}
  }

  /// Private getter for `rigid_body.shape`. Used only when printing to console.
  fn get_shape(&self) -> ShapeResult {
    self.shape
  }

  /// Private getter for `rigid_body.body`. Used only when printing to console.
  fn get_body(&self) -> Body {
    self.body
  }
}

impl From<CircleResult> for RigidBody {
  fn from(circle_result: CircleResult) -> RigidBody {
    RigidBody::new(ShapeResult::CircleResult(circle_result))
  }
}

impl fmt::Display for RigidBody {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "RigidBody({} at {})", "", self.get_body())
  }
}
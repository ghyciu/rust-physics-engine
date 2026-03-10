use std::fmt;
use crate::physics::scalar::Scalar;
use super::vector2::Vector2;

pub struct Body {
  position: Vector2,
  rotation: f32,
}

impl Body {
  pub fn new<T: Scalar>(position: Vector2, rotation: T) -> Body {
    Body {position, rotation: rotation.to_scalar()}
  }

  pub const ZERO: Body = Body {position: Vector2::ZERO, rotation: 0.0,};

  fn get_position(&self) -> Vector2 {
    self.position
  }

  fn get_rotation(&self) -> f32 {
    self.rotation
  }
}

impl Copy for Body {}
impl Clone for Body {
  fn clone(&self) -> Body {
    *self
  }
}

impl fmt::Display for Body {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Body({}, {})", self.get_position(), self.get_rotation())
  }
}
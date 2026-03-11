use std::fmt;
use crate::physics::scalar::Scalar;
use super::vector2::Vector2;

/// Describes the physics of a [`RigidBody`](super::rigid_body::RigidBody) and how they will interact with the [`World`](super::world::World).
pub struct Body {
  position: Vector2,
  rotation: f32,
}

impl Body {
  /// Create a new [`Body`] object.
  pub fn new<T: Scalar>(position: Vector2, rotation: T) -> Body {
    Body {position, rotation: rotation.to_scalar()}
  }

  /// Shorthand constructor for `Body::new(Vector2::new(0, 0), 0)`.
  pub const ZERO: Body = Body {position: Vector2::ZERO, rotation: 0.0};

  /// Return the `position` of the `Body`.
  fn get_position(&self) -> Vector2 {
    self.position
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
    write!(f, "{}", self.get_position())
  }
}
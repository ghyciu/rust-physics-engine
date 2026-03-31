use std::fmt;
use crate::math::{Vector2, Scalar};

/// Describes the physics of a [`RigidBody`](crate::physics::rigidbody::Rigidbody) and how they will interact with the [`World`](super::world::World).
#[derive(Debug, Copy, Clone, PartialEq)]
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
  pub fn get_position(&self) -> Vector2 {
    self.position
  }

}

impl fmt::Display for Body {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.get_position())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn new() {
    assert_eq!(Body::new(Vector2::new(1.0, 2.0), 3.0), Body { position: Vector2::new(1.0, 2.0), rotation: 3.0 });
  }

  #[test]
  fn get_position() {
    assert_eq!(Body::new(Vector2::new(1.0, 2.0), 3.0).get_position(), Vector2::new(1.0, 2.0));
  }

  #[test]
  fn print() {
    assert_eq!(Body::new(Vector2::new(1.0, 2.0), 3.0).to_string(), "(1, 2)");
  }
}
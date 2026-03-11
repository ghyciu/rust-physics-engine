use std::fmt;
use crate::physics::scalar::Scalar;
use super::vector2::Vector2;

/// A `Body` is any object implemented by `RigidBody` which contains a set of values for how the
/// shape will be rendered on the screen. A `RigidBody` will always have the same `Body` fields
/// regardless of the shape.
pub struct Body {
  position: Vector2,
  rotation: f32,
}

impl Body {
  /// Creates a new `Body` with the position `Vector2` and rotation `Scalar`.
  pub fn new<T: Scalar>(position: Vector2, rotation: T) -> Body {
    Body {position, rotation: rotation.to_scalar()}
  }

  /// Shorthand constructor for `Body::new(Vector2::new(0, 0), 0)`.
  pub const ZERO: Body = Body {position: Vector2::ZERO, rotation: 0.0,};

  /// Private getter for `body.position`. Used only when printing to console.
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

/// Prints the `Body` object represented as `Body(position)` in text.
impl fmt::Display for Body {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.get_position())
  }
}
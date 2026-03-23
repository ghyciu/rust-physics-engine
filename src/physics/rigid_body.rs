use std::fmt;
use crate::shape::{Shape, ShapeError, ShapeResult, ToShapeResult};
use super::body::Body;

/// Any object with physics and characteristics.
#[derive(Debug, PartialEq)]
pub struct RigidBody {
  shape: Shape,
  body: Body,
}

/// [`Result`] returned when creating a new [`RigidBody`]. May return an [`RigidBodyError`](RigidBodyError) if
/// the [`ShapeResult`] is invalid.
pub type RigidBodyResult = Result<RigidBody, RigidBodyError>;

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

#[derive(Debug, PartialEq)]
/// [`Err`] returned by [`RigidBodyResult`].
pub enum RigidBodyError {
  ShapeError(ShapeError)
}

impl From<ShapeError> for RigidBodyError {
  fn from(value: ShapeError) -> Self {
    RigidBodyError::ShapeError(value)
  }
}

#[cfg(test)]
mod tests {
  use crate::shape::circle::{Circle, CircleError, CircleRadiusError};
  use super::*;

  #[test]
  fn new() {
    let rigid_body_a: RigidBody = RigidBody::new(Circle::new(10.0)).unwrap();
    let rigid_body_b: RigidBody = RigidBody {
      shape: Shape::Circle(Circle::new(10.0).unwrap()),
      body: Body::ZERO
    };
    assert_eq!(rigid_body_a, rigid_body_b);
  }

  #[test]
  fn get_shape() {
    let rigid_body: RigidBody = RigidBody::new(Circle::new(10.0)).unwrap();
    assert_eq!(rigid_body.get_shape(), Shape::Circle(Circle::new(10.0).unwrap()));
  }

  #[test]
  fn get_body() {
    let rigid_body: RigidBody = RigidBody::new(Circle::new(10.0)).unwrap();
    assert_eq!(rigid_body.get_body(), Body::ZERO);
  }

  #[test]
  fn from_shape_error() {
    let rigid_body_error: RigidBodyError = ShapeError::CircleError(CircleError::CircleRadiusError(CircleRadiusError::NonPositiveError)).into();
    assert_eq!(RigidBodyError::ShapeError(ShapeError::CircleError(CircleError::CircleRadiusError(CircleRadiusError::NonPositiveError))), rigid_body_error);
  }

  #[test]
  fn print() {
    let rigid_body: RigidBody = RigidBody::new(Circle::new(10.0)).unwrap();
    assert_eq!(rigid_body.to_string(), "RigidBody(Circle(10) at (0, 0))");
  }
}
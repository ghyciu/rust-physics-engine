use std::fmt;
use crate::physics::Body;
use super::RigidbodyResult;
use crate::shape::{Shape, ShapeResult, ToShapeResult};

#[doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/docs/rigidbody.md"))]
#[derive(Debug, PartialEq)]
pub struct Rigidbody {
  shape: Shape,
  body: Body,
}

impl Rigidbody {
  /// Create a new [`Rigidbody`]. Returns a [`RigidbodyResult`].
  pub fn new<T: ToShapeResult>(shape_result: T) -> RigidbodyResult {
    let shape_result: ShapeResult = shape_result.to_shape_result();
    let shape: Shape = shape_result?;
    Ok(Rigidbody{shape, body: Body::ZERO})
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

impl fmt::Display for Rigidbody {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "RigidBody({} at {})", self.get_shape(), self.get_body())
  }
}

#[cfg(test)]
mod tests {
  use crate::shape::circle::Circle;
  use super::*;

  #[test]
  fn new() {
    let rigid_body_a: Rigidbody = Rigidbody::new(Circle::new(10.0)).unwrap();
    let rigid_body_b: Rigidbody = Rigidbody {
      shape: Shape::Circle(Circle::new(10.0).unwrap()),
      body: Body::ZERO
    };
    assert_eq!(rigid_body_a, rigid_body_b);
  }

  #[test]
  fn get_shape() {
    let rigid_body: Rigidbody = Rigidbody::new(Circle::new(10.0)).unwrap();
    assert_eq!(rigid_body.get_shape(), Shape::Circle(Circle::new(10.0).unwrap()));
  }

  #[test]
  fn get_body() {
    let rigid_body: Rigidbody = Rigidbody::new(Circle::new(10.0)).unwrap();
    assert_eq!(rigid_body.get_body(), Body::ZERO);
  }
}
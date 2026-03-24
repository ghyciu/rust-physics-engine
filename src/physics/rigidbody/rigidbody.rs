use std::fmt;
use crate::graphics::Renderable;
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

  /// Renders the specific [`RigidBody`] to the screen. Called by [`World`](crate::physics::World) when
  /// rendering all attached objects.
  pub(crate) fn render(&self) {
    self.get_shape().render(self.get_body().get_position());
  }
}

impl fmt::Display for Rigidbody {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Rigidbody({} at {})", self.get_shape(), self.get_body())
  }
}

#[cfg(test)]
mod tests {
  use crate::shape::circle::Circle;
  use super::*;

  #[test]
  fn new() {
    let rigidbody_a: Rigidbody = Rigidbody::new(Circle::new(10.0)).unwrap();
    let rigidbody_b: Rigidbody = Rigidbody {
      shape: Shape::Circle(Circle::new(10.0).unwrap()),
      body: Body::ZERO
    };
    assert_eq!(rigidbody_a, rigidbody_b);
  }

  #[test]
  fn get_shape() {
    let rigidbody: Rigidbody = Rigidbody::new(Circle::new(10.0)).unwrap();
    assert_eq!(rigidbody.get_shape(), Shape::Circle(Circle::new(10.0).unwrap()));
  }

  #[test]
  fn get_body() {
    let rigidbody: Rigidbody = Rigidbody::new(Circle::new(10.0)).unwrap();
    assert_eq!(rigidbody.get_body(), Body::ZERO);
  }

  #[test]
  fn print() {
    let rigidbody: Rigidbody = Rigidbody::new(Circle::new(10.0)).unwrap();
    assert_eq!(rigidbody.to_string(), "Rigidbody(Circle(10) at (0, 0))");
  }
}
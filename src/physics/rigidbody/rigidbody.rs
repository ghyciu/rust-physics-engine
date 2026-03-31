use std::fmt;
use crate::graphics::Renderer;
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
  pub(crate) fn get_shape(&self) -> Shape {
    match self.shape {
      Shape::Circle(circle) => Shape::Circle(circle),
      Shape::Line(line) => Shape::Line(line)
    }
  }

  /// Gets the `body`.
  pub(crate) fn get_body(&self) -> Body {
    self.body
  }

  /// Renders the [`RigidBody`] using the given [`Renderer`].
  pub fn render(&self, renderer: &mut dyn Renderer) {
    let body: Body = self.get_body();
    match self.get_shape() {
      Shape::Line(line) => renderer.render_line(line, body),
      Shape::Circle(circle) => renderer.render_circle(circle, body),
    }
  }
}

impl fmt::Display for Rigidbody {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Rigidbody({} at {})", self.get_shape(), self.get_body())
  }
}

#[cfg(test)]
mod tests {
  use crate::shape::line::Line;
  use crate::shape::circle::Circle;
  use crate::graphics::MockRenderer;
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
  fn render_line() {
    let rigidbody: Rigidbody = Rigidbody::new(Line::new(10.0)).unwrap();
    let mut mock_renderer: MockRenderer = MockRenderer::new();
    mock_renderer.clear_calls();
    rigidbody.render(&mut mock_renderer);
    assert_eq!(mock_renderer.get_calls().get(0).unwrap().to_string(), "Line(10)");
  }

  #[test]
  fn render_circle() {
    let rigidbody: Rigidbody = Rigidbody::new(Circle::new(10.0)).unwrap();
    let mut mock_renderer: MockRenderer = MockRenderer::new();
    rigidbody.render(&mut mock_renderer);
    assert_eq!(mock_renderer.get_calls().get(0).unwrap().to_string(), "Circle(10)");
  }

  #[test]
  fn print() {
    let rigidbody: Rigidbody = Rigidbody::new(Circle::new(10.0)).unwrap();
    assert_eq!(rigidbody.to_string(), "Rigidbody(Circle(10) at (0, 0))");
  }
}
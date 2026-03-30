use crate::graphics::Renderer;
use crate::physics::Body;
use crate::shape::line::Line;
use crate::shape::circle::Circle;

pub struct MockRenderer;

impl MockRenderer {
  pub fn new() -> MockRenderer {
    MockRenderer
  }
}

impl Renderer for MockRenderer {
  fn render_line(&mut self, line: Line, body: Body) {}
  fn render_circle(&mut self, circle: Circle, body: Body) {}
}
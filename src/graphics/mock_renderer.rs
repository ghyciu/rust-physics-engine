use crate::graphics::Renderer;
use crate::physics::Body;
use crate::shape::line::Line;
use crate::shape::circle::Circle;

pub struct MockRenderer {
  calls: Vec<String>
}

impl MockRenderer {
  pub fn new() -> MockRenderer {
    MockRenderer {
      calls: Vec::new()
    }
  }

  pub fn get_calls(&self) -> &[String] {
    self.calls.as_slice()
  }

  pub fn clear_calls(&mut self) {
    self.calls = Vec::new();
  }
}

impl Renderer for MockRenderer {
  fn render_line(&mut self, line: Line, body: Body) {
    self.calls.push(line.to_string())
  }

  fn render_circle(&mut self, circle: Circle, body: Body) {
    self.calls.push(circle.to_string())
  }
}
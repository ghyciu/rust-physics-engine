use crate::physics::Body;
use crate::shape::circle::Circle;
use crate::shape::line::Line;

pub trait Renderer {
  /// Renders a [`Line`](Line).
  fn render_line(&mut self, line: Line, body: Body);

  /// Renders a [`Circle`](Circle).
  fn render_circle(&mut self, circle: Circle, body: Body);
}
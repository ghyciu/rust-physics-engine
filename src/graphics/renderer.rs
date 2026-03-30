use crate::physics::Body;
use crate::shape::circle::Circle;
use crate::shape::line::Line;

pub trait Renderer {
  fn render_line(&mut self, line: Line, body: Body);
  fn render_circle(&mut self, circle: Circle, body: Body);
}
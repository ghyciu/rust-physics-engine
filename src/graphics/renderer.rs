use crate::physics::Body;
use crate::shape::line::Line;

pub trait Renderer {
  fn draw_line(&mut self, line: Line, body: Body);
}
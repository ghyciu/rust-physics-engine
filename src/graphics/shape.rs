use crate::graphics::line::Line;
pub enum Shape {
  Line(Line)
}

impl From<Line> for Shape {
  fn from (line: Line) -> Self {
    Shape::Line(line)
  }
}

pub trait Render {
  fn render(&self);
}

impl Render for Shape {
  fn render(&self) {
    match self {
      Shape::Line(line) => line.render()
    }
  }
}
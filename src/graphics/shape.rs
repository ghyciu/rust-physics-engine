use super::line::Line;

pub enum Shape {
  Line(Line)
}

impl From<Line> for Shape {
  fn from (line: Line) -> Self {
    Shape::Line(line)
  }
}
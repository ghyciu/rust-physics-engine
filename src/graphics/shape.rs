use super::circle::Circle;
use super::line::Line;

/// A `Shape` is any object that implements `Renderable`. An object implements `Renderable` if it
/// calls methods to draw to the `Canvas` such as `draw_line()`.
pub enum Shape {
  Line(Line),
  Circle(Circle)
}

/// Converts the `Line` object to a `Shape` object. Used to infer that a `Line` is a `Shape` such as
/// when calling `canvas.add_shape(Line)`.
impl From<Line> for Shape {
  fn from (line: Line) -> Self {
    Shape::Line(line)
  }
}

/// Converts the `Circle` object to a `Shape` object. Used to infer that a `Circle` is a `Shape` such as
/// when calling `canvas.add_shape(Circle)`.
impl From<Circle> for Shape {
  fn from (circle: Circle) -> Self {
    Shape::Circle(circle)
  }
}
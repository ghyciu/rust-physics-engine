use super::shape::Shape;

pub trait Renderable {
  fn render(&self);
}

impl Renderable for Shape {
  fn render(&self) {
    match self {
      Shape::Line(line) => line.render(),
      Shape::Circle(circle) => circle.render(),
    }
  }
}
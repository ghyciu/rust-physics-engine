use super::circle::Circle;

pub enum Shape {
  Circle(Circle)
}

impl From<Circle> for Shape {
  fn from(circle: Circle) -> Self {
    Shape::Circle(circle)
  }
}
use std::fmt;
use super::circle::Circle;

pub enum Shape {
  Circle(Circle)
}

impl From<Circle> for Shape {
  fn from(circle: Circle) -> Self {
    Shape::Circle(circle)
  }
}

impl Copy for Shape {}
impl Clone for Shape {
  fn clone(&self) -> Shape {
    match self {
      Shape::Circle(circle) => Shape::Circle(circle.clone()),
    }
  }
}

impl fmt::Display for Shape {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Shape::Circle(circle) => write!(f, "{}", circle),
    }
  }
}
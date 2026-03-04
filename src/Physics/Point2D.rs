use std::fmt;
use std::ops::Add;

pub struct Point2D {
  x: f32,
  y: f32,
}

impl Point2D {
  pub fn new(x: f32, y: f32) -> Point2D {
    Point2D { x, y }
  }
}

impl fmt::Display for Point2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl Add<Point2D> for Point2D {
  type Output = Point2D;
  fn add(self, other: Point2D) -> Point2D {
    Point2D::new(self.x + other.x, self.y + other.y)
  }
}
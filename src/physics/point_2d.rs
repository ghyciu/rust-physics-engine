use std::fmt;
use std::ops::Add;

pub struct Point2D {
  x: f32,
  y: f32,
}

pub trait Point2DValue {
  fn to_f32(self) -> f32;
}

impl Point2DValue for f32 {
  fn to_f32(self) -> f32 {
    self
  }
}

impl Point2DValue for i32 {
  fn to_f32(self) -> f32 {
    self as f32
  }
}

impl Point2D {
  pub fn new<T: Point2DValue>(x: T, y: T) -> Point2D {
    Point2D { x: x.to_f32(), y: y.to_f32() }
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
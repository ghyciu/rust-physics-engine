use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

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
  pub fn new<T: Point2DValue, U: Point2DValue>(x: T, y: U) -> Point2D {
    Point2D { x: x.to_f32(), y: y.to_f32() }
  }
}

impl Copy for Point2D {}
impl Clone for Point2D {
  fn clone(&self) -> Point2D {
    *self
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

impl Sub<Point2D> for Point2D {
  type Output = Point2D;
  fn sub(self, other: Point2D) -> Point2D {
    Point2D::new(self.x - other.x, self.y - other.y)
  }
}

impl<T: Point2DValue> Mul<T> for Point2D {
  type Output = Point2D;
  fn mul(self, other: T) -> Point2D {
    let other_f32 = other.to_f32();
    Point2D::new(self.x * other_f32, self.y * other_f32)
  }
}

impl<T: Point2DValue> Div<T> for Point2D {
  type Output = Point2D;
  fn div(self, other: T) -> Point2D {
    let other_f32 = other.to_f32();
    Point2D::new(self.x / other_f32, self.y / other_f32)
  }
}
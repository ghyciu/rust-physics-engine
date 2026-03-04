use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

pub struct Vector2 {
  x: f32,
  y: f32,
}

pub trait Vector2Value {
  fn to_f32(self) -> f32;
}

impl Vector2Value for f32 {
  fn to_f32(self) -> f32 {
    self
  }
}

impl Vector2Value for i32 {
  fn to_f32(self) -> f32 {
    self as f32
  }
}

impl Vector2 {
  pub fn new<T: Vector2Value, U: Vector2Value>(x: T, y: U) -> Vector2 {
    Vector2 { x: x.to_f32(), y: y.to_f32() }
  }

  pub fn get_x(&self) -> f32 {
    self.x
  }

  pub fn get_y(&self) -> f32 {
    self.y
  }
}

impl Copy for Vector2 {}
impl Clone for Vector2 {
  fn clone(&self) -> Vector2 {
    *self
  }
}

impl fmt::Display for Vector2 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl Add<Vector2> for Vector2 {
  type Output = Vector2;
  fn add(self, other: Vector2) -> Vector2 {
    Vector2::new(self.x + other.x, self.y + other.y)
  }
}

impl Sub<Vector2> for Vector2 {
  type Output = Vector2;
  fn sub(self, other: Vector2) -> Vector2 {
    Vector2::new(self.x - other.x, self.y - other.y)
  }
}

impl<T: Vector2Value> Mul<T> for Vector2 {
  type Output = Vector2;
  fn mul(self, other: T) -> Vector2 {
    let other_f32 = other.to_f32();
    Vector2::new(self.x * other_f32, self.y * other_f32)
  }
}

impl<T: Vector2Value> Div<T> for Vector2 {
  type Output = Vector2;
  fn div(self, other: T) -> Vector2 {
    let other_f32 = other.to_f32();
    Vector2::new(self.x / other_f32, self.y / other_f32)
  }
}

impl Eq for Vector2 {}
impl PartialEq for Vector2 {
  fn eq(&self, other: &Vector2) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl Hash for Vector2 {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.x.to_bits().hash(state);
    self.y.to_bits().hash(state);
  }
}
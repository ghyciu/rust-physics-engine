use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use crate::physics::scalar::Scalar;

pub struct Vector {
  x: f32,
  y: f32,
}

impl Vector {
  pub fn new<T: Scalar, U: Scalar>(x: T, y: U) -> Vector {
    Vector { x: x.to_f32(), y: y.to_f32() }
  }
}

impl Copy for Vector {}
impl Clone for Vector {
  fn clone(&self) -> Vector {
    *self
  }
}

impl fmt::Display for Vector {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl Add<Vector> for Vector {
  type Output = Vector;
  fn add(self, other: Vector) -> Vector {
    Vector::new(self.x + other.x, self.y + other.y)
  }
}

impl Sub<Vector> for Vector {
  type Output = Vector;
  fn sub(self, other: Vector) -> Vector {
    Vector::new(self.x - other.x, self.y - other.y)
  }
}

impl<T: Scalar> Mul<T> for Vector {
  type Output = Vector;
  fn mul(self, other: T) -> Vector {
    let other_f32 = other.to_f32();
    Vector::new(self.x * other_f32, self.y * other_f32)
  }
}

impl<T: Scalar> Div<T> for Vector {
  type Output = Vector;
  fn div(self, other: T) -> Vector {
    let other_f32 = other.to_f32();
    Vector::new(self.x / other_f32, self.y / other_f32)
  }
}

impl Eq for Vector {}
impl PartialEq for Vector {
  fn eq(&self, other: &Vector) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl Hash for Vector {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.x.to_bits().hash(state);
    self.y.to_bits().hash(state);
  }
}
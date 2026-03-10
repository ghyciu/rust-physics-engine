use super::scalar::Scalar;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

/// Represents a 2D Vector with values `x` and `y`.
/// # Construction
/// ```
/// let v: Vector2 = Vector2::new(x, y)
/// ```
pub struct Vector2 {
  x: f32,
  y: f32,
}

impl Vector2 {
  /// Creates a new `Vector2`.
  pub fn new<T: Scalar, U: Scalar>(x: T, y: U) -> Vector2 {
    Vector2 { x: x.to_scalar(), y: y.to_scalar() }
  }

  /// Shorthand constructor for ```Vector2::new(0, 0)```.
  pub const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };

  /// Public getter for `Vector2.x`.
  pub fn get_x(&self) -> f32 {
    self.x
  }

  /// Public getter for `Vector2.y`.
  pub fn get_y(&self) -> f32 {
    self.y
  }
}

/// Adds two `Vector2` together.
impl Add<Vector2> for Vector2 {
  type Output = Vector2;
  fn add(self, other: Vector2) -> Vector2 {
    Vector2::new(self.x + other.x, self.y + other.y)
  }
}

/// Subtracts a `Vector2` from a `Vector2`.
impl Sub<Vector2> for Vector2 {
  type Output = Vector2;
  fn sub(self, other: Vector2) -> Vector2 {
    Vector2::new(self.x - other.x, self.y - other.y)
  }
}

/// Multiplies a `Vector2` by a `Scalar`.
impl<T: Scalar> Mul<T> for Vector2 {
  type Output = Vector2;
  fn mul(self, scalar: T) -> Vector2 {
    let scalar = scalar.to_scalar();
    Vector2::new(self.get_x() * scalar, self.get_y() * scalar)
  }
}

/// Divides a `Vector2` by a `Scalar`.
impl<T: Scalar> Div<T> for Vector2 {
  type Output = Vector2;
  fn div(self, scalar: T) -> Vector2 {
    let scalar = scalar.to_scalar();
    Vector2::new(self.get_x() / scalar, self.get_y() / scalar)
  }
}

impl Copy for Vector2 {}
impl Clone for Vector2 {
  fn clone(&self) -> Vector2 {
    *self
  }
}

/// Prints the `Vector2` represented as `(x, y)` in text.
impl fmt::Display for Vector2 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.get_x(), self.get_y())
  }
}

/// Returns `true` if the x and y components of the two comparing `Vector2` are equal.
impl Eq for Vector2 {}
impl PartialEq for Vector2 {
  fn eq(&self, other: &Vector2) -> bool {
    self.get_x() == other.get_x() && self.get_y() == other.get_y()
  }
}

/// Allows `Vector2` to be used as a `HashMap` or `HashSet` entry.
impl Hash for Vector2 {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.get_x().to_bits().hash(state);
    self.get_y().to_bits().hash(state);
  }
}
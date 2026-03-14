use super::scalar::Scalar;
use std::fmt;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

/// 2D Vector with values `x` and `y`.
/// # Construction
/// ```
/// let v: Vector2 = Vector2::new(x, y);
/// ```
#[derive(Debug)]
pub struct Vector2 {
  x: f32,
  y: f32,
}

///
impl Vector2 {
  /// Creates a new [`Vector2`].
  pub fn new<T: Scalar, U: Scalar>(x: T, y: U) -> Vector2 {
    Vector2 { x: x.to_scalar(), y: y.to_scalar() }
  }

  /// Shorthand constructor for ```Vector2::new(0, 0)```.
  pub const ZERO: Vector2 = Vector2 { x: 0.0, y: 0.0 };

  /// Returns the `x` of the [`Vector2`].
  pub fn get_x(&self) -> f32 {
    self.x
  }

  /// Returns the `y` of the [`Vector2`].
  pub fn get_y(&self) -> f32 {
    self.y
  }
}

/// Implementation for adding two [`Vector2`] together.
impl Add<Vector2> for Vector2 {
  type Output = Vector2;
  fn add(self, other: Vector2) -> Vector2 {
    Vector2::new(self.x + other.x, self.y + other.y)
  }
}

/// Implementation for subtracting a [`Vector2`] by a `Vector2`.
impl Sub<Vector2> for Vector2 {
  type Output = Vector2;
  fn sub(self, other: Vector2) -> Vector2 {
    Vector2::new(self.x - other.x, self.y - other.y)
  }
}

/// Implementation for multiplying a [`Vector2`] by a [`Scalar`].
impl<T: Scalar> Mul<T> for Vector2 {
  type Output = Vector2;
  fn mul(self, scalar: T) -> Vector2 {
    let scalar = scalar.to_scalar();
    Vector2::new(self.get_x() * scalar, self.get_y() * scalar)
  }
}

/// Implementation for dividing a [`Vector2`] by a [`Scalar`].
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

impl fmt::Display for Vector2 {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.get_x(), self.get_y())
  }
}
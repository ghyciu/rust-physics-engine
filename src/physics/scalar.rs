/// Represents an `i32` or `f32` used in `Vector2` instantiation, when multiplying or dividing a
/// `Vector2` by a factor.
pub trait Scalar {
  fn to_scalar(self) -> f32;
}

impl Scalar for f32 {
  fn to_scalar(self) -> f32 {
    self
  }
}

impl Scalar for i32 {
  fn to_scalar(self) -> f32 {
    self as f32
  }
}
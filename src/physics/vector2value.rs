/// Represents an `i32` or `f32` used in `Vector2` instantiation, or when multiplying or dividing a
/// `Vector2` by a scalar.
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
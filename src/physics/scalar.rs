/// Describes a numeric value. Used when creating a [`Vector2`] or when multiplying or dividing a `Vector2`
/// by some numeric factor.
pub trait Scalar {
  /// Converts the numeric value to a [`Scalar`].
  fn to_scalar(self) -> f32;
}

/// Implementation for converting a `f32` to a [`Scalar`].
impl Scalar for f32 {
  /// Converts the `f32` to a [`Scalar`].
  fn to_scalar(self) -> f32 {
    self
  }
}

/// Implementation for converting an `i32` type to a [`Scalar`].
impl Scalar for i32 {
  /// Converts the `i32` to a [`Scalar`].
  fn to_scalar(self) -> f32 {
    self as f32
  }
}
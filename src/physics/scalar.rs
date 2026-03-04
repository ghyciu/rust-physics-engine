pub trait Scalar {
  fn to_f32(self) -> f32;
}

impl Scalar for f32 {
  fn to_f32(self) -> f32 {
    self
  }
}

impl Scalar for i32 {
  fn to_f32(self) -> f32 {
    self as f32
  }
}
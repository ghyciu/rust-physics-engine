use crate::physics::scalar::Scalar;

pub struct Circle {
  radius: f32,
}

impl Circle {
  pub fn new<T: Scalar>(radius: T) -> Self {
    Circle {radius: radius.to_scalar()}
  }
}
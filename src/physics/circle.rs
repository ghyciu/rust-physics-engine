use std::fmt;
use crate::physics::rigidbody::RigidBody;
use crate::physics::scalar::Scalar;

pub struct Circle {
  radius: f32,
}

impl Circle {
  pub fn new<T: Scalar>(radius: T) -> Self {
    Circle {radius: radius.to_scalar()}
  }
  
  fn get_radius(&self) -> f32 {
    self.radius
  }
}

impl Copy for Circle {}
impl Clone for Circle {
  fn clone(&self) -> Circle {
    *self
  }
}

impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle({})", self.get_radius())
  }
}
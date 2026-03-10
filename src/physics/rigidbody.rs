use std::fmt;
use super::body::Body;
use super::shape::Shape;
use super::circle::Circle;

pub struct RigidBody {
  shape: Shape,
  body: Body,
}

impl RigidBody {
  pub fn new<T: Into<Shape>>(shape: T) -> RigidBody {
    RigidBody { body: Body::ZERO, shape: shape.into() }
  }

  fn get_shape(&self) -> Shape {
    self.shape
  }

  fn get_body(&self) -> Body {
    self.body
  }
}

impl From<Circle> for RigidBody {
  fn from(circle: Circle) -> Self {
    RigidBody::new(Shape::Circle(circle))
  }
}

impl fmt::Display for RigidBody {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "RigidBody({}: {})", self.get_shape(), self.get_body())
  }
}
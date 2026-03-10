use std::fmt;
use super::body::Body;
use super::shape::Shape;
use super::circle::Circle;

/// A `RigidBody` is any component with a general set of values in `Body` and a specific set of values
/// `Shape` on how they will be rendered on the screen.
pub struct RigidBody {
  shape: Shape,
  body: Body,
}

impl RigidBody {
  /// Creates a new `RigidBody` with the passed `Shape`.
  pub fn new<T: Into<Shape>>(shape: T) -> RigidBody {
    RigidBody { body: Body::ZERO, shape: shape.into() }
  }

  /// Private getter for `rigidbody.shape`. Used only when printing to console.
  fn get_shape(&self) -> Shape {
    self.shape
  }

  /// Private getter for `rigidbody.body`. Used only when printing to console.
  fn get_body(&self) -> Body {
    self.body
  }
}

/// Creates a new `RigidBody` and infers that the passed `Circle` is a type of `Shape`.
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
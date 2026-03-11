use crate::physics::circle::CircleResult;
use crate::physics::shape::Shape;

pub type ShapeResult = Result<Shape, &'static str>;
pub trait ToShapeResult {
  fn to_shape_result(&self) -> ShapeResult;
}

impl ToShapeResult for CircleResult {
  fn to_shape_result(&self) -> ShapeResult {
    self.map(Shape::Circle)
  }
}
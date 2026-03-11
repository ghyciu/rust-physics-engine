use crate::physics::circle::CircleResult;

pub enum ShapeResult {
  CircleResult(CircleResult)
}

impl From<CircleResult> for ShapeResult {
  fn from(result: CircleResult) -> ShapeResult {
    ShapeResult::CircleResult(result)
  }
}

impl Copy for ShapeResult {}
impl Clone for ShapeResult {
  fn clone(&self) -> ShapeResult {
    match self {
      ShapeResult::CircleResult(circle) => ShapeResult::CircleResult(circle.clone()),
    }
  }
}
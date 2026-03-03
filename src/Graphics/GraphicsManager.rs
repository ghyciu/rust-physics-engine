use crate::Graphics::Shape::Shape;
use crate::Graphics::Square::Square;

pub struct GraphicsManager {
  shapes: Vec<Box<dyn Shape>>
}

impl GraphicsManager {
  pub fn new() -> GraphicsManager {
    GraphicsManager { shapes: Vec::new() }
  }

  pub fn add_shape(&mut self, shape: Square) {
    self.shapes.push(Box::new(shape));
  }

  pub fn render(&self) {
    for shape in &self.shapes {
      shape.render();
    }
  }
}
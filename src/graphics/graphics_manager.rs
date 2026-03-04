use crate::graphics::shape::{Render, Shape};
use crate::graphics::square::Square;

pub struct GraphicsManager {
  shapes: Vec<Shape>
}

impl GraphicsManager {
  pub fn new() -> GraphicsManager {
    GraphicsManager { shapes: Vec::new() }
  }

  pub fn add_shape<T: Into<Shape>>(&mut self, shape: T) {
    self.shapes.push(shape.into());
  }

  pub fn render(&self) {
    for shape in &self.shapes {
      shape.render();
    }
  }
}
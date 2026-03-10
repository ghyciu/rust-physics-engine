use crate::graphics::renderable::Renderable;
use super::shape::Shape;

pub struct Canvas {
  shapes: Vec<Shape>
}

impl Canvas {
  pub fn new() -> Canvas {
    Canvas { shapes: Vec::new() }
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
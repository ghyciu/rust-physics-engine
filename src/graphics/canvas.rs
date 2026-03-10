use super::renderable::Renderable;
use super::shape::Shape;

/// A container for all the renderable `Shapes`.
pub struct Canvas {
  shapes: Vec<Shape>
}

impl Canvas {
  /// Creates a new `Canvas`.
  pub fn new() -> Canvas {
    Canvas { shapes: Vec::new() }
  }

  /// Adds a `Shape` to the `Canvas`.
  pub fn add_shape<T: Into<Shape>>(&mut self, shape: T) {
    self.shapes.push(shape.into());
  }

  /// Calls the `render()` method of all `Shapes` in the `Canvas`.
  pub fn render(&self) {
    for shape in &self.shapes {
      shape.render();
    }
  }
}
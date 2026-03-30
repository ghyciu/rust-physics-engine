use crate::graphics::{Renderer};
use crate::physics::rigidbody::Rigidbody;

/// Describes the world interacted by [`RigidBody`](crate::physics::rigidbody::Rigidbody) objects.
pub struct World {
  objects: Vec<Rigidbody>
}

impl World {
  // World Constants
  pub const MIN_AREA: f32 = 0.01_f32 * 0.01_f32;
  pub const MAX_AREA: f32 = 64_f32 * 64_f32;

  pub const MIN_DENSITY: f32 = 0.5_f32;
  pub const MAX_DENSITY: f32 = 20_f32;

  pub fn new() -> World {
    World {objects: Vec::new()}
  }

  pub fn add_object(&mut self, object: Rigidbody) {
    self.objects.push(object);
  }

  pub fn render(&self, renderer: &mut dyn Renderer) {
    for object in &self.objects {
      object.render(renderer);
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::graphics::MockRenderer;
  use crate::physics::rigidbody::Rigidbody;
  use crate::physics::World;
  use crate::shape::circle::Circle;

  #[test]
  fn new() {
    let world: World = World::new();
    assert_eq!(world.objects.len(), 0);
  }

  #[test]
  fn add_object() {
    let mut world: World = World::new();
    let rigidbody: Rigidbody = Rigidbody::new(Circle::new(10.0)).unwrap();
    world.add_object(rigidbody);
    assert_eq!(world.objects.len(), 1);
  }

  #[test]
  fn render() {
    let mut world: World = World::new();
    let rigidbody: Rigidbody = Rigidbody::new(Circle::new(10.0)).unwrap();
    let mut mock_renderer: MockRenderer = MockRenderer::new();
    world.add_object(rigidbody);
    world.render(&mut mock_renderer);
  }
}
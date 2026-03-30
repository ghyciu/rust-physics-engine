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
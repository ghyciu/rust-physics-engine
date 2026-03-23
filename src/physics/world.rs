/// Describes the world interacted by [`RigidBody`](crate::physics::rigidbody::Rigidbody) objects.
pub struct World {

}

impl World {
  pub const MIN_AREA: f32 = 0.01_f32 * 0.01_f32;
  pub const MAX_AREA: f32 = 64_f32 * 64_f32;

  pub const MIN_DENSITY: f32 = 0.5_f32;
  pub const MAX_DENSITY: f32 = 20_f32;
}
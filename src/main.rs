
use rust_physics_engine::physics::RigidBody;
use rust_physics_engine::shape::circle::Circle;

fn main() {
  print!("{:?}", RigidBody::new(Circle::new(-5)));
}

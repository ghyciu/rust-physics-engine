use rust_physics_engine::shape::circle::Circle;

mod physics;

fn main() {
  print!("{:?}", RigidBody::new(Circle::new(-5)));
}

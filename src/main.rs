
use crate::physics::circle::Circle;
use crate::physics::rigid_body::RigidBody;

mod physics;

fn main() {
  print!("{:?}", RigidBody::new(Circle::new(-5)));
}

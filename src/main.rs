use crate::physics::rigid_body::RigidBody;
use crate::physics::circle::Circle;

mod physics;

fn main() {
  print!("{}", RigidBody::new(Circle::new(2)));
}

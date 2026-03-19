use super::circle::CircleError;

/// [`Err`] returned by [`ShapeResult`](crate::shape::ShapeResult). A variant of [`RigidBodyResult`](crate::physics::RigidBodyResult).
#[derive(Debug)]
pub enum ShapeError {
  CircleError(CircleError)
}
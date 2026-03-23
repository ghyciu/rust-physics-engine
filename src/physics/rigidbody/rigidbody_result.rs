use crate::physics::rigidbody::{Rigidbody, RigidbodyError};

/// [`Result`] returned when creating a new [`RigidBody`]. May return an [`RigidBodyError`](RigidBodyError) if
/// the [`ShapeResult`] is invalid.
pub type RigidbodyResult = Result<Rigidbody, RigidbodyError>;
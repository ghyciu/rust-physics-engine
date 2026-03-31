use crate::physics::rigidbody::{Rigidbody, RigidbodyError};

/// [`Result`] returned when creating a new [`Rigidbody`]. May return an [`RigidbodyError`] if
/// the [`ShapeResult`](crate::shape::ShapeResult) is invalid.
pub type RigidbodyResult = Result<Rigidbody, RigidbodyError>;
use super::{Circle, CircleError};

/// [`Result`] returned when creating a new [`Circle`]. May return a [`CircleError`]
/// if [`LengthResult`](crate::math::length::LengthResult) is invalid.
pub type CircleResult = Result<Circle, CircleError>;
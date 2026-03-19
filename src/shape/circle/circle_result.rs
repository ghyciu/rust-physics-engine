use super::{Circle, CircleError};

/// [`Result`] returned when creating a new [`Circle`]. May return a [`CircleError`]
/// if [`CircleRadiusResult`](super::CircleRadiusResult) is invalid.
pub type CircleResult = Result<Circle, CircleError>;
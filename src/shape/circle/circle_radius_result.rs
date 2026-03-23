use super::{CircleRadius, CircleRadiusError};

/// [`Result`] returned when creating a new [`CircleRadius`]. May return
/// a [`CircleRadiusError`] if invalid. A variant of [`CircleResult`](super::CircleResult).
pub type CircleRadiusResult = Result<CircleRadius, CircleRadiusError>;
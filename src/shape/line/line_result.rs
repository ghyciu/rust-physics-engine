use super::{Line, LineError};

/// [`LineResult`] returned when creating a new [`Line`]. Returns a [`LineError`]
/// if [`LengthResult`](crate::math::length::LengthResult) is invalid.
pub type LineResult = Result<Line, LineError>;
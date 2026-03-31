use super::{Length, LengthError};

/// [`Result`] returned when creating a new [`Length`]. Returns a
/// [`LengthError`] if the passed value is non-positive.
pub type LengthResult = Result<Length, LengthError>;
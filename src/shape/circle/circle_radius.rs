use super::{CircleRadiusError, CircleRadiusResult};

/// Validated `radius` of a [`Circle`](super::Circle).
#[derive(Debug, Copy, Clone)]
pub struct CircleRadius(f32);



impl CircleRadius {
  /// Creates a new [`CircleRadius`] object. Returns a [`CircleRadiusResult`].
  pub fn new(value: f32) -> CircleRadiusResult{
    if value <= 0_f32 {
      return Err(CircleRadiusError::NonPositiveError);
    }
    Ok(CircleRadius(value))
  }

  /// Returns the value of [`CircleRadius`].
  pub fn get(&self) -> f32 {
    self.0
  }
}
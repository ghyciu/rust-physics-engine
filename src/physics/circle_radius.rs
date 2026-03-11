pub struct CircleRadius(f32);

impl CircleRadius {
  pub fn new(value: f32) -> Result<Self, &'static str> {
    if value < 0_f32 {
      return Err("Radius must be positive")
    }
    Ok(CircleRadius(value))
  }

  pub fn get(&self) -> f32 {
    self.0
  }
}

impl Copy for CircleRadius {}
impl Clone for CircleRadius {
  fn clone(&self) -> CircleRadius {
    *self
  }
}
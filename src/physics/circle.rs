use std::fmt;
use super::scalar::Scalar;

/// Describes a circle with radius `radius`.
/// # Construction
/// ```
/// let circle = Circle::new(5)
/// ```
pub struct Circle {
  radius: CircleRadius,
}

/// [`Result`] object returned when creating a new circle. May return an [`Err`]
/// if [`CircleRadiusResult`] is invalid.
pub type CircleResult = Result<Circle, &'static str>;

impl Circle {
  /// Create a new [`Circle`] object. Returns a [`CircleResult`].
  pub fn new<T: Scalar>(radius: T) -> CircleResult {
    let radius: CircleRadius = CircleRadius::new(radius.to_scalar())?;
    Ok(Circle{ radius })
  }

  /// Get the `radius` of the [`Circle`]. Used only when printing to console.
  fn get_radius(&self) -> f32 {
    self.radius.get()
  }
}

impl Copy for Circle {}
impl Clone for Circle {
  fn clone(&self) -> Circle {
    *self
  }
}

impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle({})", self.get_radius())
  }
}

/// Describes a validated `radius` of a [`Circle`] object.
struct CircleRadius(f32);

/// [`Result`] object returned when creating a new circle radius. May return
/// an [`Err`] if the value of the radius is invalid.
type CircleRadiusResult = Result<CircleRadius, &'static str>;

impl CircleRadius {
  /// Create a new [`CircleRadius`] object. Returns a [`CircleRadiusResult`].
  pub fn new(value: f32) -> CircleRadiusResult{
    if value <= 0_f32 {
      return Err("Radius must be positive")
    }
    Ok(CircleRadius(value))
  }

  /// Return the value of [`CircleRadius`].
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
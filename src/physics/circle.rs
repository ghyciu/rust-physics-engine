use std::fmt;
use super::scalar::Scalar;

/// Circle with radius `radius`.
/// # Construction
/// ```
/// let circle = Circle::new(5)
/// ```
#[derive(Debug, Copy, Clone)]
pub struct Circle {
  radius: CircleRadius,
}

/// [`Result`] returned when creating a new [`Circle`]. May return a [`CircleError`]
/// if [`CircleRadiusResult`] is invalid.
pub(crate) type CircleResult = Result<Circle, CircleError>;

impl Circle {
  /// Creates a new [`Circle`] object. Returns a [`CircleResult`].
  pub fn new<T: Scalar>(radius: T) -> CircleResult {
    let radius: CircleRadius = CircleRadius::new(radius.to_scalar())?;
    Ok(Circle{ radius })
  }

  /// Gets the `radius` of the [`Circle`]. Used only when printing to console.
  fn get_radius(&self) -> f32 {
    self.radius.get()
  }
}

impl fmt::Display for Circle {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Circle({})", self.get_radius())
  }
}

/// Validated `radius` of a [`Circle`] object.
#[derive(Debug, Copy, Clone)]
struct CircleRadius(f32);

/// [`Result`] returned when creating a new [`CircleRadius`]. May return
/// a [`CircleRadiusError`] if invalid.
type CircleRadiusResult = Result<CircleRadius, CircleRadiusError>;

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


/// [`Err`] returned by [`CircleRadiusResult`].
#[derive(Debug)]
pub(crate) enum CircleRadiusError {
  /// [`CircleRadius`] is not positive.
  NonPositiveError,
}

impl fmt::Display for CircleRadiusError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      CircleRadiusError::NonPositiveError => write!(f, "Radius cannot be negative"),
    }
  }
}

impl std::error::Error for CircleRadiusError{}

/// [`Err`] returned by [`CircleResult`].
#[derive(Debug)]
pub(crate) enum CircleError {
  /// The `radius` is invalid.
  CircleRadiusError(CircleRadiusError)
}

impl From<CircleRadiusError> for CircleError {
  fn from(value: CircleRadiusError) -> Self {
    CircleError::CircleRadiusError(value)
  }
}
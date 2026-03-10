use super::vector2::Vector2;

pub struct Math {}

impl Math {
  pub fn length(v: Vector2) -> f32 {
    let dx: f32 = v.get_x();
    let dy: f32 = v.get_y();
    return ((dx * dx) + (dy * dy)).sqrt();
  }

  pub fn distance(u: Vector2, v: Vector2) -> f32 {
    let dx: f32 = u.get_x() - v.get_x();
    let dy: f32 = u.get_y() - v.get_y();
    return ((dx * dx) + (dy * dy)).sqrt();
  }

  pub fn normalize(v: Vector2) -> Vector2 {
    let length: f32 = Math::length(v);
    return Vector2::new(v.get_x() / length, v.get_y() / length);
  }

  pub fn dot(u: Vector2, v: Vector2) -> f32 {
    return u.get_x() * v.get_x() + u.get_y() * v.get_y();
  }

  pub fn cross(u: Vector2, v: Vector2) -> f32 {
    return u.get_x() * v.get_y() - u.get_y() * v.get_x();
  }
}
use macroquad::prelude::*;
use crate::graphics::Renderer;
use crate::math::Vector2;
use crate::physics::Body;
use crate::shape::line::Line;

#[derive(Copy, Clone)]
pub struct MacroquadRenderer;

impl MacroquadRenderer {
  pub fn new() -> MacroquadRenderer {
    MacroquadRenderer
  }
}

impl Renderer for MacroquadRenderer {
  fn render_line(&mut self, line: Line, body: Body) {
    let position: Vector2 = body.get_position();
    let position_x: f32 = position.get_x();
    let position_y: f32 = position.get_y();

    let length_x: f32 = line.get_length();
    let length_y: f32 = line.get_length();

    let screen_height: f32 = screen_height();

    draw_line(position_x, screen_height - position_y, position_x + length_x, screen_height - position_y - length_y, 1.0_f32, BLACK);
  }

  fn render_circle(&mut self, circle: crate::shape::circle::Circle, body: Body) {
    todo!()
  }
}
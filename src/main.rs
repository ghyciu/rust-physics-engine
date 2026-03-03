use macroquad::prelude::*;

#[macroquad::main("Physics Engine")]
async fn main() {
  loop {
    next_frame().await
  }
}

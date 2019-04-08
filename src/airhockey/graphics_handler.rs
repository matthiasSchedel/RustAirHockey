use super::{ball, ball::Ball};
use crate::graphics::graphics::Graphics;

const DRAW_LAYER_NO: u8 = 1;

pub struct GraphicsHandler {
  graphics: Graphics,
  screen_size: [u16; 2],
}

impl GraphicsHandler {
  pub fn new(graphics: Graphics) -> GraphicsHandler {
    GraphicsHandler {
      graphics: graphics,
      screen_size: [480, 272],
    }
  }
  // Ball constructor

 pub fn draw_ball(&self, color: u32, pos: [u16;2], radius: u16) {
    self.graphics_controller.draw_circle(
     color, pos, radius, false, color);
  }

  pub fn draw_playing_field(&self) {}

  pub fn draw_player(&self, color: Color, pos: [u16;2], radius: u16){
    self.graphics_controller.draw_circle(color, pos, radius, false, color);
  }
}

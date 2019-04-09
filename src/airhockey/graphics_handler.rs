//! graphics handler module
use crate::graphics::graphics::Graphics;

/// no of layer where to draw
const DRAW_LAYER_NO: u8 = 1;

/// graphics handler struct
pub struct GraphicsHandler {
  graphics: Graphics,
  screen_size: [u16; 2],
}

impl GraphicsHandler {
  /// handler constructor
  pub fn new(graphics: Graphics) -> GraphicsHandler {
    GraphicsHandler {
      graphics: graphics,
      screen_size: [480, 272],
    }
  }

  /// draw the ball
  pub fn draw_ball(&self, color: u32, pos: [u16; 2], radius: u16) {
    self.graphics.draw_circle(color, pos, radius, false, color);
  }

  /// draw the playing field
  pub fn draw_playing_field(&self) {}

  /// draw the player
  pub fn draw_player(&self, color: u32, pos: [u16; 2], radius: u16) {
    self.graphics.draw_circle(color, pos, radius, false, color);
  }
}

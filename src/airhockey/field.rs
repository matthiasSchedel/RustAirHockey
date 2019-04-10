//! Airhockey game field.
use crate::airhockey::init::Handler;
/// field max width
pub const WIDTH_MAX: u16 = 480;
/// field max height
pub const HEIGHT_MAX: u16 = 272;
/// field background color
pub const BACKGROUND_COLOR: u32 = 0x00ffff;
pub const BORDER_WIDTH: u16 = 10;
pub const GOAL_SIZE: u16 = 50;

/// structrepresents a Field in airhockey game
pub struct Field {}
impl Field {
  /// Field constructor
  pub fn new() -> Field {
    Field {}
  }

  pub fn draw(&self, handler: &mut Handler) {
    handler.graphics_handler.draw_playing_field();
  }
}

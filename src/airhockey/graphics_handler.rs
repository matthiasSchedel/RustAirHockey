use super::{ball, ball::Ball};
use crate::graphics::graphics::GraphicsController;

const DRAW_LAYER_NO: u8 = 1;

pub struct GraphicsHandler<'a> {
  graphics: &'a GraphicsController<'static>,
  screen_size: [u16; 2],
}

impl<'a> GraphicsHandler<'a> {
  pub fn new(graphics: &'a GraphicsController<'static>) -> GraphicsHandler<'a> {
    GraphicsHandler {
      graphics: graphics,
      screen_size: [480, 272],
    }
  }
  // Ball constructor

  pub fn draw_ball(&self, ball_: Ball) {
    self.graphics.draw_circle(
      ball::COLOR,
      ball_.position,
      ball::RADIUS,
      ball::HAS_STROKE,
      ball::STROKE_COLOR,
    );
  }

  pub fn draw_playing_field(&self) {}
}

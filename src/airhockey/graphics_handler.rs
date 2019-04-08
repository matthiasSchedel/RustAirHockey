use super::{ball, ball::Ball};
use super::field;
use super::{player, player::Player};
use crate::graphics::graphics::Graphics;

const DRAW_LAYER_NO: u8 = 1;

pub struct GraphicsHandler<'a> {
  graphics: &'a Graphics<'static>,
  screen_size: [u16; 2],
}

impl<'a> GraphicsHandler<'a> {
  pub fn new(graphics: &'a Graphics<'static>) -> GraphicsHandler<'a> {
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

pub fn draw_player(self,player_:Player){
  self.graphics.draw_circle(
    player::COLOR.
    player_.position,
    player::RADIUS,
    player::HAS_STROKE,
    player::STROKE_COLOR,
  );
}


  pub fn draw_playing_field(&self) {
    self.graphics.draw_field(
      field::BACKGROUND_COLOR,
      [field::WIDTH_MAX,field::HEIGHT_MAX],
      field::BORDER_WIDTH,
      field::GOAL_SIZE,
    );
  }
}

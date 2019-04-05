use super::{ball::self, ball::Ball};
use crate::graphics_controller::graphics_controller::GraphicsController;

const DRAW_LAYER_NO: u8 = 1;

pub struct GraphicsHandler {
  graphics_controller: GraphicsController
}

impl GraphicsHandler {
    pub fn new(graphics_controller: GraphicsController) {

  } 
  // Ball constructor

  pub fn draw_ball(&self, ball_: Ball) {
      self.graphics_controller.draw_circle(ball::COLOR, ball_.position, ball::RADIUS, ball::HAS_STROKE,  ball::STROKE_COLOR);
  }
  
}

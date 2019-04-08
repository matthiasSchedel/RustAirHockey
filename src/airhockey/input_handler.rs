use super::{ball, ball::Ball};
use crate::input::input::Input;

const DRAW_LAYER_NO: u8 = 1;

pub struct InputHandler<'a> {
  input: &'a Input<'static>,
  screen_size: [u16; 2],
}

impl<'a> InputHandler<'a> {
  pub fn new(input: &'a Input<'static>) -> InputHandler<'a> {
    InputHandler {
      input: input,
      screen_size: [480, 272],
    }
  }
  // Ball constructor

  pub fn calculate_new_pos_from_input(pos: [u16; 2]) -> (bool, [u16; 2]) {
    return { (false, [0, 0]) };
  }
}

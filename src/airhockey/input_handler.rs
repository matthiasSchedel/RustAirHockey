extern crate alloc;
use super::helper;
use super::field;
use alloc::vec::Vec;
use arrayvec::ArrayVec;
use crate::touch::{self, Touch};

use super::ball::{self, Ball};
use crate::input::input::Input;

const DRAW_LAYER_NO: u8 = 1;

pub struct InputHandler {
  input: Input,
  screen_size: [u16; 2],
}

impl InputHandler {
  pub fn new(input: Input) -> InputHandler {
    InputHandler {
      input: input,
      screen_size: [480, 272],
    }
  }
  

  pub fn calculate_new_pos_from_input(pos: (u16,u16)) -> (bool, (u16,u16)) {
    return { (false, (0,0)) };
  }

  ///Get touch position of a player by aggregating over all relevant touches
  pub fn get_target_position(& mut self, pos:(u16,u16), touch_radius: u16, half_x_min: u16, half_x_max:u16) ->(u16,u16) {
      let filtered_touches = self.filter_touches_for_player(half_x_min, half_x_max, self.input.get_touch_positions());
      let mut position = (0,0);
        
      for touch in filtered_touches {
          //If the touch position is inside the radius of the player
          if helper::unsigned_subtraction(pos.0, touch.0) < 2*touch_radius
          && helper::unsigned_subtraction(pos.1, touch.1) < 2*touch_radius {
              position.0 = helper::average_vector([touch.0, position.0].to_vec());
              position.1 = helper::average_vector([touch.1, position.1].to_vec());
                
          }
          else{
              //TODO choose position closest to player
              //Choose abtrirary touch position (will be the last value in the list)
              position.0 = touch.0;
              position.1 = touch.1;
          }
      }
      (position.0, position.1)
    }

    ///Filter a list of touches for a players
    fn filter_touches_for_player(& mut self, x_min:u16, x_max:u16, inputs: Vec<(u16,u16)>) ->Vec<(u16,u16)> {
        let mut positions: Vec<(u16, u16)> = Vec::new();
        for input in inputs {
            //If the touch position is in the player's half of the field
            if input.0 < x_max && input.0 > x_min{
                positions.push((input.0, input.1));
            } else{
                continue;
            } 
        }
        positions
    }
}

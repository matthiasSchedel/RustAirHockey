//! input handler module
extern crate alloc;
use super::field;
use super::helper;
use crate::touch::{self, Touch};
use alloc::vec::Vec;
use arrayvec::ArrayVec;

use super::ball::{self, Ball};
use crate::input::input::Input;

/// layer to draw in
const DRAW_LAYER_NO: u8 = 1;

/// input handler struct
pub struct InputHandler {
    input: Input,
    screen_size: [u16; 2],
    last_input_positions: Vec<(u16, u16)>,
}

impl InputHandler {
    pub fn new(input: Input) -> InputHandler {
        InputHandler {
            input: input,
            screen_size: [480, 272],
            last_input_positions: Vec::new(),
        }
    }

    ///Get touch position of a player by aggregating over all relevant touches
    pub fn get_target_position(
        &mut self,
        pos: (u16, u16),
        touch_radius: u16,
        half_x_min: u16,
        half_x_max: u16,
    ) -> (bool, (u16, u16)) {
        let filtered_touches = self.filter_touches_for_player(half_x_min, half_x_max);
        let mut touch_min_dist: f64 = 1000.0; 
        let mut position = (
            (half_x_max - half_x_min) / 2 + half_x_min,
            (self.screen_size[1] / 2) as u16,
        );
        let mut is_touch = false;

        for touch in filtered_touches {
            is_touch = true;
            //If the touch position is inside the radius of the player
            // if helper::unsigned_subtraction(pos.0, touch.0) < 2 * touch_radius
            //     && helper::unsigned_subtraction(pos.1, touch.1) < 2 * touch_radius
            // {
            //     position.0 = touch.0; //helper::average_vector([touch.0, position.0].to_vec());
            //     position.1 = touch.1; // helper::average_vector([touch.1, position.1].to_vec());
            // } else {
            //     //TODO choose position closest to player
            //     //Choose abtrirary touch position (will be the last value in the list)
            //     position.0 = touch.0;
            //     position.1 = touch.1;
            //     // is_touch = false;
            // }
            let touch_curr_dist : f64 = helper::calculate_point_distance(pos, touch);
            if touch_curr_dist < touch_min_dist {
                touch_min_dist = touch_curr_dist;
                position = touch;
            }
        }
        (is_touch, (position.0, position.1))
    }
    pub fn fetch_input(&mut self) {
        self.last_input_positions = self.input.get_touch_positions()
    }

    ///Filter a list of touches for a players
    fn filter_touches_for_player(&mut self, x_min: u16, x_max: u16) -> Vec<(u16, u16)> {
        let mut positions: Vec<(u16, u16)> = Vec::new();
        for input in &mut self.last_input_positions {
            //If the touch position is in the player's half of the field
            if input.0 < x_max && input.0 > x_min {
                positions.push((input.0, input.1));
            } else {
                continue;
            }
        }
        positions
    }
}

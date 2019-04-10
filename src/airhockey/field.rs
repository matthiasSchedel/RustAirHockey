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

const x_min:i32 = 10;//i32::from(BORDER_WIDTH);
const x_max:i32 = 470;//i32::from(WIDTH_MAX) - i32::from(BORDER_WIDTH);
const y_min:i32 = 10;//i32::from(BORDER_WIDTH);
const y_max:i32 = 262;//i32::from(HEIGHT_MAX) - i32::from(BORDER_WIDTH);

/// struct represents a Field in airhockey game
pub struct Field {
  /*   x_min:i32,
    x_max:i32,
    y_min:i32,
    y_max:i32, */
}
impl Field {
    /// Field constructor
    pub fn new() -> Field {
        Field {
          /*   x_min:i32::from(BORDER_WIDTH),
            x_max:i32::from(WIDTH_MAX) - i32::from(BORDER_WIDTH),
            y_min:i32::from(BORDER_WIDTH),
            y_max:i32::from(HEIGHT_MAX) - i32::from(BORDER_WIDTH), */
        }
    }

    pub fn draw(&self, handler: &mut Handler) {
      handler.graphics_handler.draw_playing_field();
    } 
}
///Checks whether a circle with center x,y and radius would be still within the field
pub fn inside_field_including_radius(pos: (i32, i32), radius: u16)-> bool{      
    let radius_test = i32::from(radius);
    let x_test_min = i32::from(pos.0) - radius_test;
    let x_test_max = i32::from(pos.0) + radius_test;
    let y_test_min = i32::from(pos.1) - radius_test;
    let y_test_max = i32::from(pos.1) + radius_test;
        
    x_test_min >= x_min 
        && x_test_max <= x_max 
        && y_test_min >= y_min
        && y_test_max <= y_max
}
/// Find the position that is on the line between current and target position but still in the field
pub fn get_position_still_in_field(target_pos: (i32, i32), current_pos: (u16, u16), radius:u16)-> (u16, u16){
    let mut pos = (0,0);
    let mut a = 0;
    let radius_test = i32::from(radius);
    if inside_field_including_radius(target_pos, radius){
        (target_pos.0 as u16, target_pos.1 as u16)
    } else{
        let r_x = target_pos.0 - i32::from(current_pos.0);
        let r_y = target_pos.1 - i32::from(current_pos.1);
        //If the target is below the x border
        if target_pos.0 - radius_test < x_min {
            pos.0 = x_min;
            a = (pos.0- target_pos.0)/ r_x;
            pos.1 = target_pos.1 + r_y * a;
        }
        //If the target is above the x border
        if target_pos.0 + radius_test > x_max {
            pos.0 = x_max;
            a = (pos.0- target_pos.0)/ r_x;
            pos.1 = target_pos.1 + r_y * a;
        }
        //If the target is below the y border
        if target_pos.1 - radius_test < y_min{
            pos.1 = y_min;
            a = (pos.1- target_pos.1)/ r_x;
            pos.0 = target_pos.0 + r_y * a;
        }
        //If the target is above the y border
        if target_pos.1 + radius_test > y_max{
            pos.1 = y_max;
            a = (pos.1- target_pos.1)/ r_x;
            pos.0 = target_pos.0 + r_y * a;
        }
        (pos.0 as u16, pos.1 as u16)
    }
}

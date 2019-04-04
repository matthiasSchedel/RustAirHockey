//! Airhockey Player.

use super::field;

// General player properties
pub const PLAYER_RADIUS: u16 = 15;
const PLAYER_COLOR: u8 = 150;

// Player
pub struct Player {
    player_id: u8,
    xpos_centre: u16,
    ypos_centre: u16,
    radius:u16,
    //* The last valid touch position
    touch_pos_x: u16,
    touch_pos_y: u16,
    x_min: u16,
    x_max: u16,
    
}
impl Player {
    //Create new player

    pub fn new(player_id: u8) -> Player{
        //This has to be changed if more than two players exist
        if player_id == 0 {
            Player {
                player_id: player_id,
                xpos_centre : field::WIDTH_MAX/4,
                ypos_centre : field::HEIGHT_MAX/2,
                radius: PLAYER_RADIUS,
                touch_pos_x:0,
                touch_pos_y:0,
                x_min: 0,
                x_max:field::WIDTH_MAX/2,
            }
        }
        else{
            Player {
                player_id : player_id,
                xpos_centre : 3* field::WIDTH_MAX/4,
                ypos_centre : field::HEIGHT_MAX/2,
                radius: PLAYER_RADIUS,
                touch_pos_x:0,
                touch_pos_y:0,
                x_min : (field::WIDTH_MAX/2)+1,
                x_max : field::WIDTH_MAX-1,
            }
        }
    }
    pub fn get_position(&self) -> (u16, u16) {
        return (self.xpos_centre, self.ypos_centre);
    }

    pub fn set_position(& mut self, x : u16, y:u16){
        self.xpos_centre = x;
        self.ypos_centre = y;
    }
    //*Update the last valid touch_position
    pub fn update_player_on_touch(& mut self, x: u16, y: u16){
        self.touch_pos_x = x;
        self.touch_pos_y = y;
        self.move_player();
    }
    //*Move the player according to the touch position
    //TODO implement delayed movement
    fn move_player(& mut self){
        self.xpos_centre = self.touch_pos_x;
        self.ypos_centre = self.touch_pos_y;
    }

    //*Whether a x-position is in the right half of the field of a player
    pub fn correct_half(&self, x: u16, y: u16)->bool{
        x < self.x_max && x > self.x_min
    }
    pub fn get_id(&self) -> u8{
        self.player_id
    }
}
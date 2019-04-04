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
            }
        }
        else{
            Player {
                player_id : player_id,
                xpos_centre : 3* field::WIDTH_MAX/4,
                ypos_centre : field::HEIGHT_MAX/2,
                radius: PLAYER_RADIUS,
            }
        }
    }
    //*Draw the player at a given position
    pub fn draw_player_at_position(&self) {
    }

    pub fn get_position(&self) -> (u16, u16) {
        return (self.xpos_centre, self.ypos_centre);
    }

    pub fn set_position(& mut self, x : u16, y:u16){
        self.xpos_centre = x;
        self.ypos_centre = y;
    }
}
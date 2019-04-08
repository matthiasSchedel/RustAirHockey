//! Airhockey Player.

use super::field;

// General player properties
pub const RADIUS: u16 = 10;
pub const COLOR: u32 = 150;
pub const STROKE_COLOR: u32 = 0xfff000; 
pub const HAS_STROKE: bool = true; 


// Player
pub struct Player {
    pub player_id: u8,
    pub position: [u16;2],
}

impl Player {
    //Create new player

    pub fn new(player_id: u8) -> Player{
        //This has to be changed if more than two players exist
        if player_id == 0 {
            Player {
                player_id: player_id,
                position : [field::WIDTH_MAX/4,field::HEIGHT_MAX/2],   
            }
        }
        else{
            Player {
                player_id : player_id,
                position : [3* field::WIDTH_MAX,field::HEIGHT_MAX],
            }
        }
    }
    
    pub fn get_position(&self) -> (u16, u16) {
        return (self.position[0], self.position[1]);
    }
}
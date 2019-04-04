//! Airhockey game.

extern crate alloc;


use super::field;
use super::player::{self, Player};
use alloc::vec::Vec;
use arrayvec::ArrayVec;
use crate::touch::{self, Touch};

pub struct Game {
    players: Vec<Player>,
}
impl Game {
  pub fn new(number_players : u8) -> Game{ 
      let mut players = Vec::new();
      for p in 0..number_players {
         players.push(Player::new(p)) 
      }

      Game {
          players: players,
      }
  }
  //*Find out whether a player is touched
  pub fn is_touched(&self, p_id : usize, touches: &ArrayVec<[Touch; 5]>) -> bool{
        let (x,y) : (u16, u16) = self.players[p_id].get_position();
        let mut touched: bool = false;
        for touch in touches {
            
            if unsigned_subtraction(x, touch.x) < 2*player::PLAYER_RADIUS 
            && unsigned_subtraction(y, touch.y) < 2*player::PLAYER_RADIUS {
                println!("p{} ist touched", p_id);
                return true;
            }
            else{
                touched = false;
            }
        }
    touched
  }
    pub fn init(&self) {
        
    }
}
//* Perform subtraction on unsigned values (absolute difference)
pub fn unsigned_subtraction(x: u16, y:u16) -> u16{
    if x < y {
        y-x
    } else{
        x-y
    }
} 


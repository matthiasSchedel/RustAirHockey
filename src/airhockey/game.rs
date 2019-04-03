//! Airhockey game.

extern crate alloc;


use super::field;
use super::player::{self, Player};
use alloc::vec::Vec;
use stm32f7_discovery::touch;

pub struct Game {
    players: Vec<Player>
}
impl Game {
  pub fn new(number_players : u8) -> Game{ 
      let players = Vec::new();
      for p in 0..number_players {
         players.push(Player::new(p)) 
      }

      Game {
          players: players,
      }
  }  
  pub fn is_touched(&self, p_id : usize, touches: ArrayVec<[Touch; 5]) -> bool{
    let (x,y) : (u16, u16) = self.players[p_id].get_position();
    for touch in &touch::touches(&mut i2c_3).unwrap() {
        if ((touch.x - x).abs() < 2*player::PLAYER_RADIUS) && ((touch.y - y).abs()< 2*player::PLAYER_RADIUS){
            println!("p{} ist touched", p_id);
            return true;
        }
        else{
           return false; 
        }
    }
    return false;
  }
}




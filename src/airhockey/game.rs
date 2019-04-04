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

  /* pub fn update_players (&self){
      for p in self.players {
          self.aggregate_touch_positions(p,touches: &ArrayVec<[Touch; 5]>);
      }

  } */
  //TODO only use valid (per player) touch positions

   //*Get touch position of a player by aggregating over all relevant touches
   //*And set the player touch position accordingly
  fn aggregate_touch_positions(& mut self, p: Player, touches: &ArrayVec<[Touch; 5]>) {
        let (x,y) : (u16, u16) = p.get_position();
        let mut touched: bool = false;
        let mut x_positions: Vec<u16> = Vec::new();
        let mut y_positions: Vec<u16> = Vec::new();
        for touch in touches {
            if unsigned_subtraction(x, touch.x) < 2*player::PLAYER_RADIUS 
            && unsigned_subtraction(y, touch.y) < 2*player::PLAYER_RADIUS {
                println!("p{} ist touched", p.get_id());
                x_positions.push(touch.x);
                y_positions.push(touch.y);
                touched= true;
            }
            else{
                touched = false;
            }
        }
        let mut touch_x: u16 = 0;
        let mut touch_y: u16 = 0;
        if touched{
            touch_x = average_vector(x_positions);
            touch_y = average_vector(y_positions);    
        }
        else{
            //Choose abtrirary touch position
            //TODO must be in the correct field!!
            touch_x = touches[0].x;
            touch_y = touches[0].y;
        }
        p.update_player_on_touch(touch_x, touch_y);
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

//*Average over values in a vector
pub fn average_vector(values: Vec<u16>) -> u16{
    let mut sum: u16 = 0;
    let mut count: u16 = 0;
    for x in values {
        sum = sum + x;
        count = count +1;
    }
    sum/count
}


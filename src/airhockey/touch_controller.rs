extern crate alloc;

use super::field;
use super::player::{self, Player};
use alloc::vec::Vec;
use arrayvec::ArrayVec;
use crate::touch::{self, Touch};

///
pub struct TouchController {
    players: &Vec<Player>,
}
impl TouchController {
    ///
    pub fn new(&players: Vec<Player>) -> TouchController{ 
        let mut players = Vec::new();
        for p in 0..number_players {
            players.push(Player::new(p)) 
        }
        TouchController {
            players: players,
        }
    }
    pub fn init(&self) {
        
    }
    //TODO this should be done in the player class
    ///Update the last valid touch_position
    pub fn update_players_touch(& mut self){
        let touch_pos = (0,0);
        for p_id in 0..self.players.len(){
            touch_pos = aggregate_touch_positions(p_id, filter_touches_for_player(p_id, touches));
            self.players[p_id].move_player(touch_pos.0, touch_pos.1);
        }        
        
    }
    ///Get touch position of a player by aggregating over all relevant touches
    fn aggregate_touch_positions(& mut self, p_id: usize, touches: Vec<(u16,u16)>) ->(u16,u16) {
        let p = &self.players[p_id];
        let (x,y) : (u16, u16) = p.get_position();
        //let mut touched = false;
        //let mut positions = Vec::new();
        let mut position_x = 0;
        let mut position_y = 0;
        
        for touch in touches {
            //If the touch position is inside the radius of the player
            if unsigned_subtraction(x, touch.0) < 2*player::PLAYER_RADIUS
            && unsigned_subtraction(y, touch.1) < 2*player::PLAYER_RADIUS {
                position_x = average_vector([touch.0, position_x].to_vec());
                position_y = average_vector([touch.1, position_y].to_vec());
                println!("p{} ist touched", p.get_id());
                
            }
            else{
                //Choose abtrirary touch position (will be the last value in the list)
                position_x = touch.0;
                position_y = touch.1;
            }
        }
        (position_x, position_y)
    }
    
    ///Filter a list of touches for a player
    fn filter_touches_for_player(& mut self, p_id: usize, touches: &ArrayVec<[Touch; 5]>) ->Vec<(u16,u16)> {
        let p = &self.players[p_id];
        let mut positions: Vec<(u16, u16)> = Vec::new();
        for touch in touches {
            //If the touch position is in the player's half of the field
            if p.correct_half(touch.x){
                positions.push((touch.x, touch.y));
            } else{
                continue;
            } 
        }
        positions
    }
}


/// Perform subtraction on unsigned values (absolute difference)
pub fn unsigned_subtraction(x: u16, y:u16) -> u16{
    if x < y {
        y-x
    } else{
        x-y
    }
} 

///Average over values in a vector
pub fn average_vector(values: Vec<u16>) -> u16{
    let mut sum: u16 = 0;
    let mut count: u16 = 0;
    for x in values {
        sum += x;
        count += 1;
    }
    sum/count
}

///Converts a list of tuples into a tuple of lists
pub fn get_tuple_of_lists<T>(list_of_tuples: Vec<(T,T)>) -> (Vec<T>,Vec<T>){
    let mut vector_0 = Vec::new();
    let mut vector_1 = Vec::new();
    for tuple in list_of_tuples {
        vector_0.push(tuple.0);
        vector_1.push(tuple.1);
    }
    (vector_0,vector_1)
}

/* pub fn create_tuple_list_from_two_lists<T>(vector_1: Vec<T>, vector_2: Vec<T>) -> Vec<(T,T)> {
    let mut vector_tuples = Vec::new();
    for (v_1, v_2) in (vector_1, vector_2){
        vector_tuples.push((v_1, v_2));
    }
    vector_tuples
} */


//! Airhockey game.
extern crate alloc;

use super::field;
use super::player;
use super::player::Player;
use alloc::vec::Vec;

pub struct Game {
    players: Vec<Player>,
}
impl Game {
    // game constructor
    pub fn new(number_players: u8) -> Game {
        let mut players: Vec<Player> = Vec::new();
        for p in 0..number_players {
            players.push(Player::new(p))
        }
        Game { players: players }
    }
    // is touched method
    pub fn is_touched(&self, p_id: usize) -> bool {
        self.players[p_id].get_position();
        return false;
    }

    pub fn init(&self) {
        
    }
}

use crate::alloc;
use alloc::vec::Vec;
use super::init::Handler;

pub const POINTS_PER_GOAL: u8 = 1;

pub struct Score<'a> {
    player_scores: Vec<u16>,
    max_score: u16,
    handler:&'a Handler
}

impl<'a> Score<'a> {
    // game constructor
    pub fn new(number_players: u8, max_score: u16, handler:&'a Handler) -> Score<'a> {
        let mut player_scores: Vec<u16> = Vec::new();
        for p in 0..number_players {
            player_scores.push(0);
        }
        Score {
            player_scores: player_scores,
            max_score: max_score,
            handler: handler
        }

    }
    pub fn add_score(&self, player:u8) {

    }
    
    // return 0 if no player has won else player number
    pub fn is_game_over(&self) -> (bool, u8) {
        let mut i:u16 = 1;
        // for p in self.player_scores {
        //     if (p >= self.max_score) 
        //     {
        //         return (true, p as u8);
        //     } 
        //     i += 1;
        // }
        return (false,0);
    }

}

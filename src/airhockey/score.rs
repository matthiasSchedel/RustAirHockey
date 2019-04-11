//! score module
use super::init::Handler;
use crate::alloc;
use alloc::vec::Vec;

/// points scored per goal
pub const POINTS_PER_GOAL: u8 = 1;
pub const NUMBER_WIDTH: u16 = 25;
pub const NUMBER_HIGHT: u16 = 40;
pub const DOT_COLOR: u32 = 0x000000;

/// define the x coord of left right right goal used for goal checking
const GOAL_X_POSITIONS: [u16; 2] = [15, 470];

/// score struct in airhockey game
pub struct Score {
    player_scores: Vec<u16>,
    max_score: u16,
}

impl Score {
    /// game constructor
    pub fn new(number_players: u8, max_score: u16) -> Score {
        let mut player_scores: Vec<u16> = Vec::new();
        for p in 0..number_players {
            player_scores.push(0);
        }
        Score {
            player_scores: player_scores,
            max_score: max_score,
        }
    }
    /// add to the score of a player
    pub fn add_score(&mut self, player: u8) {
        if self.player_scores[player as usize] < 9 {
            self.player_scores[player as usize] = self.player_scores[player as usize] + 1;
        }
    }

    /// return 0 if no player has won else player number
    pub fn is_game_over(&mut self) -> (bool, u8) {
        let mut i: u16 = 1;
        for p in 0..self.player_scores.len() {
            if p >= 9 as usize {
                return (true, p as u8);
            }
            i += 1;
        }
        return (false, 0);
    }

    pub fn draw(&self, handler: &mut Handler) {
        let score = self.player_scores.clone();
        handler.graphics_handler.draw_score(score);
    }

    pub fn check_goals_and_update_score(&mut self, pos: [u16; 2]) -> (bool, u8) {
        if pos[0] < GOAL_X_POSITIONS[0] {
            self.player_scores[0] = self.player_scores[0] + 1;
            (true, 0)
        } else if pos[1] > GOAL_X_POSITIONS[1] {
            self.player_scores[1] = self.player_scores[1] + 1;
            (true, 1)
        } else {
            (false, 0)
        }
    }
}

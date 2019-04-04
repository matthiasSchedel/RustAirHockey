//! Airhockey game.

use super::{controller::Controller, player::Player, score::Score};
use alloc::vec::Vec;

const POINTS_PER_GOAL:u8 = 1;

pub struct Game {
    players: Vec<Player>,
    controller: Controller,
    score: Score
}
impl Game {
    // game constructor
    pub fn new(number_players: u8, controller: Controller) -> Game {
        let mut players: Vec<Player> = Vec::new();
        for p in 0..number_players {
            players.push(Player::new(p))
        }
        Game {
            players: players,
            controller: controller,
        }

    }
    // is touched method
    pub fn is_touched(&self, p_id: usize) -> bool {
        self.players[p_id].get_position();
        return false;
    }

    pub fn start(&self, max_score:u16, ball_speed:u16, use_gravity:bool) {
        self.score = score::new(self.players.len(),max_score);
        
    }
    pub fn game_loop (&self) {
            loop {
            

            self.handle_inputs();
            self.handle_physics();
            let scored:u8 = self.evaluate_score();
            if  scored != 0 {
                self.score.add_score(self.evaluate_score(), POINTS_PER_GOAL);
                let who_won:u8 = self.is_game_over();
                if  who_won { break; }
            }
            self.handle_graphcis();
        }
    }

    fn check_win_condition() -> bool {
           if self.score.is_game_over() == 0 
            {

            } else {
                // print player self.score.is_game_over() has won
            }
    }

    fn handle_inputs() {
        self.input.handle_gui_inputs(); // pause or other controls 
        self.input.handle_player_inputs(); // handle all player inputs
    }

    fn handle_physics() {
        self.physics.handle_physics();
    }

    fn render(&self) {

        for p in self.players {
        self.controller.graphics_controller.draw_circle(p.color, p.get_position()[0])
        } 
    }

    fn collisions(&self) {

    }




    // pub fn init(&self) {

    // }
}

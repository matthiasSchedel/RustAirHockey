//! Airhockey game.
const MAX_SCORE:u16 = 10;

use super::{player::Player, score::Score, ball::Ball, init::Handler, init};
use alloc::vec::Vec;

const POINTS_PER_GOAL: u8 = 1;
const COLOR_ARRAY: [u32; 4] = [0xfff000, 0xfff000, 0xfff000, 0xfff000];


fn createGameFromElements<'a>(number_players: u8) -> Game<'a>{
    let handler = init::createHandler();
    let ball = Ball::new(&handler);
    let mut players: Vec<Player> = Vec::new();
        for p in 0..number_players {
            players.push(Player::new(p,&handler))
        }
    let score = Score::new(players.len() as u8, MAX_SCORE, &handler);
    
    return Game { ball: ball, players: players, score: score };
 }


pub struct Game<'a> {
    players: Vec<Player<'a>>,
    score: Score<'a>,
    ball:Ball<'a>
}
impl<'a> Game<'a> {
    // game constructor
    pub fn new(number_players: u8) -> Game<'a> {
        let game: Game = createGameFromElements(number_players) ;
        return game;
    }
    // is touched method
    pub fn is_touched(&self, p_id: usize) -> bool {
        self.players[p_id].get_position();
        return false;
    }

    pub fn start(&self, max_score: u16, ball_speed: u16, use_gravity: bool) {
        // self.score = Score::new(self.players.len() as u8,max_score);
        false;
    }
    pub fn game_loop(&self) -> bool {
            // self.handle_inputs();
            // self.handle_physics();
            let scored: u8 = self.evaluate_score();
            if scored != 0 {
                self.score.add_score(self.evaluate_score());
                if self.score.is_game_over().0 {
                    return false;
                }
            }
            return true;
            // self.handle_graphcis();
    }

    fn check_win_condition(&self) -> bool {
        if self.score.is_game_over().0 {
            return true;
        } else {
            return false;
            // print player self.score.is_game_over() has won
        }
    }

    fn handle_inputs(&self) {
        // self.input.handle_gui_inputs(); // pause or other controls
        // self.input.handle_player_inputs(); // handle all player inputs
    }

    fn handle_physics(&self) {
        // self.physics.handle_physics();
    }

    fn evaluate_score(&self) -> u8 {
        return 0;
    }

    fn render(&self) {

        // for p in self.players {
        // self.controller.graphics.draw_circle(COLOR_ARRAY[(p.player_id as usize)], p.get_position().0)
        // }
    }

    fn collisions(&self) {}

    // pub fn init(&self) {

    // }
}

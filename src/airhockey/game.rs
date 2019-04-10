//! Airhockey game.
const MAX_SCORE: u16 = 10;

use super::{
    super::physics::physics, ball::Ball, field, field::Field, init, init::Handler, player::Player,
    score::Score,
};
use alloc::vec::Vec;

/// points scored per goal
const POINTS_PER_GOAL: u8 = 1;
/// player colors array
const COLOR_ARRAY: [u32; 4] = [0xfff000, 0xfff000, 0xfff000, 0xfff000];

/// game struct
pub struct Game {
    players: Vec<Player>,
    score: Score,
    ball: Ball,
    field: Field,
    handler: Handler,
}
impl Game {
    /// game constructor
    pub fn new(number_players: u8, handler: Handler) -> Game {
        let ball = Ball::new();
        let mut players: Vec<Player> = Vec::new();
        let field = Field::new();
        for p in 0..number_players {
            players.push(Player::new(p))
        }
        let score = Score::new(players.len() as u8, MAX_SCORE);

        return Game {
            ball: ball,
            players: players,
            score: score,
            field: field,
            handler: handler,
        };
    }
    /// is touched method
    pub fn is_touched(&self, p_id: usize) -> bool {
        self.players[p_id].get_position();
        return false;
    }

    ///
    pub fn check_goals() -> bool {
        return false;
    }

    fn handle_game_over(&self) {}

    /// game loop
    pub fn game_loop(&mut self) -> ! {
        loop {
            // handle score
            let scored: bool = self.update_score();
            if scored {
                if self.score.is_game_over().0 {
                    self.handle_game_over();
                    //gehe in einen anderen State
                    loop {}
                } else {
                    self.score.draw(&mut self.handler);
                    // score board updaten
                }
            }
            //input handling
            //update players with new user input -> new player pos
            // self.handle_inputs();
            self.update_players_with_user_input();

            //collision handling
            self.handle_collisions();

            //graphics handling
            self.prepare_drawing();
            self.draw_field();
            self.draw_score();
            self.draw_players();
            self.draw_ball();
        }

        // self.handle_graphcis();
    }

    fn prepare_drawing(&self) {
        
    }

    fn update_score(&mut self) -> bool {
        let score_res = self.score.check_goals_and_update_score(self.ball.position);
        return (score_res.0);
    }

    fn draw_field(&mut self) {
        self.field.draw(&mut self.handler);
    }

    /// update player with user input
    fn update_players_with_user_input(&mut self) {
        for p in &mut self.players {
            p.update_on_user_input(&mut self.handler);
            p.move_player();
        }
    }

    /// check ball for colls
    // constructs a physics-object from the current game state, checks for collision und updates ball position and speed
    fn check_ball_for_collisons(&mut self, mut handler: Handler) {
        handler
            .physics_handler
            .physics
            .set_ball_pos(&self.ball.position[0], &self.ball.position[1]);
        handler
            .physics_handler
            .physics
            .set_ball_speed(&self.ball.speed[0], &self.ball.speed[1]);
        let mut active_player: usize = 1;
        if self.ball.position[0] < (field::WIDTH_MAX / 2) {
            active_player = 0;
        }

        handler.physics_handler.physics.update_ball_position(
            self.players[active_player].get_position().0,
            self.players[active_player].get_position().1,
            /*active_player.radius*/ 10,
            f32::from(self.players[active_player].get_speed().0),
            f32::from(self.players[active_player].get_speed().1),
        );
    }

    ///
    fn draw_ball(&mut self) {
        self.ball.draw(&mut self.handler);
    }

    fn draw_score(&mut self) {}

    fn draw_players(&mut self) {
        for p in &mut self.players {
            p.draw(&mut self.handler);
        }
    }

    /// check if a player has won and return winning player if true
    fn check_win_condition(&mut self) -> bool {
        if self.score.is_game_over().0 {
            return true;
        } else {
            return false;
            // print player self.score.is_game_over() has won
        }
    }

    /// handler the player inputs
    fn handle_inputs(&self) {
        // self.input.handle_gui_inputs(); // pause or other controls
        // self.input.handle_player_inputs(); // handle all player inputs
    }

    /// handle the physics
    fn handle_physics(&self) {
        // self.physics.handle_physics();
    }

    // /// evaluate the score
    // fn update_score(&self) -> (bool, u8) {
    //     self.score.check_goals_and_update_score(self.ball.position);
    //     self.score.draw_score();
    //     return (false, 0);
    // }

    /// render the game
    fn render(&self) {

        // for p in self.players {
        // self.controller.graphics.draw_circle(COLOR_ARRAY[(p.player_id as usize)], p.get_position().0)
        // }
    }

    /// get the collsiiosn
    fn handle_collisions(&self) {}

    // pub fn init(&self) {

    // }
}

//! Airhockey game.
const MAX_SCORE: u16 = 10;

use super::{
    super::physics::physics, ball::Ball, field, init, init::Handler, player::Player, score::Score};
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
    handler: Handler,
}
impl Game {
    /// game constructor
    pub fn new(number_players: u8, handler: Handler) -> Game {
        let ball = Ball::new();
        let mut players: Vec<Player> = Vec::new();
        for p in 0..number_players {
            players.push(Player::new(p))
        }
        let score = Score::new(players.len() as u8, MAX_SCORE);

        return Game {
            ball: ball,
            players: players,
            score: score,
            handler: handler,
        };
    }
    /// is touched method
    pub fn is_touched(&self, p_id: usize) -> bool {
        self.players[p_id].get_position();
        return false;
    }

    /// game loop
    pub fn game_loop(&self) -> ! {
        // self.handle_inputs();
        // self.handle_physics();
        loop {
            // handle score
            let scored: (bool, u8) = self.evaluate_score();
            if scored.0 {
                if self.score.is_game_over().0 {
                    //gehe in anderean State
                    loop {}
                } else {
                    // score board updaten
                }
            }
            //input handling
            //update players with new user input -> new player pos
            //collision handling
            //check ball for collision -> new ball pos

            //graphics handling
            //draw field
            //draw score
            //draw ball
            //draw players
        }

        // self.handle_graphcis();
    }

    /// update player with user input
    fn update_players_with_user_input(&self) {
        //rufe methoden in klasse player auf
    }

    /// check ball for colls
    // constructs a physics-object from the current game state, checks for collision und updates ball position and speed
    fn check_ball_for_collisons(&self) {

        // construct physics-object
        let physics = physics::Physics::new(
            field::WIDTH_MAX,
            field::HEIGHT_MAX,
            /*Ball::RADIUS*/ 10,
        );

        // give current ball state as borrow
        physics.set_ball_pos(&self.ball.position[0], &self.ball.position[1]);
        physics.set_ball_speed(&self.ball.speed[0], &self.ball.speed[1]);
        
        // find active player - we can not collide with player on the far side of the field
        let active_player = if self.ball.position[0] < field::WIDTH_MAX / 2 {
            self.players[0]
        } else {
            self.players[1]
        };


        physics.update_ball_position(
            active_player.get_position().0,
            active_player.get_position().1,
            /*active_player.radius*/ 10,
            f32::from(active_player.get_speed().0),
            f32::from(active_player.get_speed().1),
        );
    }

    /// draw field
    fn draw_field(&self) {}

    /// draw score
    fn draw_score(&self) {}

    /// draw ball
    fn draw_ball(&self) {}

    /// draw player
    fn draw_players(&self) {}

    /// check if a player has won and return winning player if true
    fn check_win_condition(&self) -> bool {
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

    /// evaluate the score
    fn evaluate_score(&self) -> (bool, u8) {
        return (false, 0);
    }

    /// render the game
    fn render(&self) {

        // for p in self.players {
        // self.controller.graphics.draw_circle(COLOR_ARRAY[(p.player_id as usize)], p.get_position().0)
        // }
    }

    /// get the collsiiosn
    fn collisions(&self) {}

    // pub fn init(&self) {

    // }
}

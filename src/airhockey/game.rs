//! Airhockey game.
const MAX_SCORE: u16 = 10;

use super::{
    ball, ball::Ball, field, field::Field, init, init::Handler, player::Player, score::Score,
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

        Game {
            ball,
            players,
            score,
            field,
            handler,
        }
    }

    fn get_drawable_objects(&mut self) -> Vec<((u16, u16), u16, u32)> {
        let mut drawables: Vec<((u16, u16), u16, u32)> = Vec::new();
        for p in &mut self.players {
            drawables.push((p.get_position(), p.get_radius(), p.get_color()));
        }
        drawables.push((
            (self.ball.position[0], self.ball.position[1]),
            ball::RADIUS,
            ball::COLOR,
        ));
        drawables
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

    fn handle_game_over(&mut self, winning_player: u8) {
        self.handler.graphics_handler.draw_game_over(winning_player);
    }

    /// game loop
    pub fn game_loop(&mut self) -> ! {
        loop {
            //input handling
            //update players with new user input -> new player pos
            self.handle_inputs();
            self.update_players_with_user_input();

            //collision handling
            self.handle_collisions();

            //graphics handling
            self.prepare_drawing();
            self.draw_field();
            self.draw_score();
            // handle score
            let scored: bool = self.update_score();
            if scored {
                let (is_over, winning_player) = self.score.is_game_over();
                if is_over {
                    self.handle_game_over(winning_player);
                    //gehe in einen anderen State

                    loop {}
                } else {
                    self.score.draw(&mut self.handler);
                    // score board updaten
                }
            }
        }

        // self.handle_graphcis();
    }

    fn prepare_drawing(&mut self) {
        let drawables = self.get_drawable_objects();
        self.handler.graphics_handler.draw_game_content(&drawables);
    }

    fn update_score(&mut self) -> bool {
        self.score
            .check_goals_and_update_score(self.ball.position)
            .0
    }

    /// update player with user input
    fn update_players_with_user_input(&mut self) {
        for p in &mut self.players {
            p.update_on_user_input(&mut self.handler);
            p.move_player();
        }
    }

    /// draw score
    fn draw_field(&mut self) {
        self.field.draw(&mut self.handler);
    }

    fn draw_score(&mut self) {}

    /// check if a player has won and return winning player if true
    fn check_win_condition(&mut self) -> bool {
        self.score.is_game_over().0
    }

    /// handler the player inputs
    fn handle_inputs(&self) {
        // self.input.handle_gui_inputs(); // pause or other controls
        // self.input.handle_player_inputs(); // handle all player inputs
    }

    /// get the collsiiosn
    fn handle_collisions(&mut self) {
        let active_player = if self.ball.position[0] < (field::WIDTH_MAX / 2) {
            0
        } else {
            1
        };
        let ball_speed = (self.ball.speed[0], self.ball.speed[1]);
        let player_position = self.players[active_player].get_position();
        let player_radius = self.players[active_player].get_radius();
        let player_speed = (
            f64::from(self.players[active_player].get_speed().0),
            f64::from(self.players[active_player].get_speed().1),
        );

        self.handler.physics_handler.update_ball_speed(&ball_speed);
        let new_ball_position = self.handler.physics_handler.update_ball_position(
            player_position,
            player_radius,
            player_speed,
        );
        let new_ball_speed = self.handler.physics_handler.get_ball_speed();

        self.ball.position = [new_ball_position.0, new_ball_position.1];
        self.ball.speed = [new_ball_speed.0, new_ball_speed.1]
    }
}

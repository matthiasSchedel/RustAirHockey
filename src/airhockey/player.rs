//! Airhockey Player.
use super::helper;
/// player radius
pub const RADIUS: u16 = 20;
/// player fill color
pub const COLOR: u32 = 0xfff000;
/// player radius
pub const STROKE_COLOR: u32 = 0xfff000;
/// has the player stroke?
pub const HAS_STROKE: bool = false;

use super::field;
use super::graphics_handler::GraphicsHandler;
use super::input_handler::{self, InputHandler};

use super::init::Handler;

/// Player
pub struct Player {
    player_id: u8,
    current_pos: (u16, u16),
    radius: u16,
    color: u32,
    ///Defining the half of the field where the player can move
    x_min: u16,
    x_max: u16,
    ///The player is following the user's input (given by target_position)ar
    target_pos: (u16, u16),
    ///The speed the player is moving towards the target position
    speed: (i32, i32),
}

impl Player {
    /// Create new player
    pub fn new(player_id: u8) -> Player {
        let radius = RADIUS;
        let color = COLOR;
        //This has to be changed if more than two players exist
        if player_id == 0 {
            Player {
                player_id: player_id,
                current_pos: (field::WIDTH_MAX / 4, field::HEIGHT_MAX / 2),
                radius: radius,
                color: color,
                x_min: 0,
                x_max: field::WIDTH_MAX / 2,
                ///The target position is the same as the current position at initialization
                target_pos: (field::WIDTH_MAX / 4, field::HEIGHT_MAX / 2),
                speed: (0, 0),
            }
        } else {
            //TODO zusammenfassen!
            Player {
                player_id: player_id,
                current_pos: (3 * field::WIDTH_MAX / 4, field::HEIGHT_MAX / 2),
                radius: radius,
                color: color,
                x_min: (field::WIDTH_MAX / 2) + 1,
                x_max: field::WIDTH_MAX - 1,
                ///The target position is the same as the current position at initialization
                target_pos: (3 * field::WIDTH_MAX / 4, field::HEIGHT_MAX / 2),
                speed: (0, 0),
            }
        }
    }
    ///Checks for user input and updates the player accordingly
    ///To be called in the game's main loop
    pub fn update_player(&mut self, handler: &mut Handler) {
        self.update_on_user_input(handler);
        self.move_player();
        self.draw(handler);
    }

    // //draw the player
    // fn draw(&self, handler: &Handler) {
    //     handler.graphics_handler.draw_player(
    //         self.color,
    //         [self.current_pos.0, self.current_pos.1],
    //         self.radius,
    //     );

    pub fn draw(&self, handler: &mut Handler) {
        handler.graphics_handler.draw_player(
            self.color,
            [self.current_pos.0, self.current_pos.1],
            self.radius,
        );
    }

    ///Move the player according to the target position
    pub fn move_player(&mut self) {
        if i32::from(self.current_pos.0) - i32::from(self.target_pos.0) <= self.speed.0
            && i32::from(self.current_pos.1) - i32::from(self.target_pos.1) <= self.speed.1
        {
            self.speed = (0, 0);
            self.current_pos = self.target_pos;
        } else {
            self.current_pos = (
                (i32::from(self.current_pos.0) + self.speed.0) as u16,
                (i32::from(self.current_pos.1) + self.speed.1) as u16,
            );
        }
    }

    ///update player on user input
    pub fn update_on_user_input(&mut self, handler: &mut Handler) {
        if self.player_id == 0 {
            handler.input_handler.fetch_input();
        }
        let (is_touched, pos) = handler.input_handler.get_target_position(
            self.current_pos,
            self.radius,
            self.x_min,
            self.x_max,
        );
        //Important! Only perform when there are touches!
        if is_touched {
            self.target_pos = pos;
        }
        self.speed = (
            (i32::from(self.target_pos.0) - i32::from(self.current_pos.0)) / 20,
            (i32::from(self.target_pos.1) - i32::from(self.current_pos.1)) / 20,
        );
    }
    ///Get the player id
    pub fn get_id(&self) -> u8 {
        self.player_id
    }
    ///Get the current position of the player
    pub fn get_position(&self) -> (u16, u16) {
        (self.current_pos.0, self.current_pos.1)
    }

    ///set the current position of the player
    pub fn set_position(&mut self, x: u16, y: u16) {
        self.current_pos.0 = x;
        self.current_pos.1 = y;
    }

    pub fn get_speed(&self) -> (i32, i32) {
        self.speed
    }

    pub fn setColor(&mut self, color: u32) {
        self.color = color;
    }
    pub fn setRadius(&mut self, radius: u16) {
        self.radius = radius;
    }
}

//! Airhockey Player.
use super::helper;
pub const RADIUS: u16 = 10;
pub const COLOR: u32 = 0xfff000;
pub const STROKE_COLOR: u32 = 0xfff000;
pub const HAS_STROKE: bool = false;

use super::field;
use super::input_handler::{self, InputHandler};
use super::graphics_handler::GraphicsHandler;

// General player properties
//pub const PLAYER_RADIUS: u16 = 15;
//pub const PLAYER_COLOR: u8 = 150;

// Player
pub struct Player<'a> {
    player_id: u8,
    current_pos_x: u16,
    current_pos_y: u16,
    radius:u16,
    color: u32,
    ///Defining the half of the field where the player can move
    x_min: u16,
    x_max: u16,
    ///The player is following the user's input (given by target_position)
    target_pos_x: u16,
    target_pos_y: u16,
    ///The speed the player is moving towards the target position
    speed_x: u16,
    speed_y: u16,
    input_handler: &'a InputHandler,
    graphics_handler: &'a GraphicsHandler,
}

impl<'a> Player<'a> {
    //Create new player

    pub fn new(player_id: u8, radius: u16, color: u32, input_handler: InputHandler, graphics_handler:GraphicsHandler) -> Player<'a>{
        //This has to be changed if more than two players exist
        if player_id == 0 {
            Player {
                player_id: player_id,
                current_pos_x : field::WIDTH_MAX/4,
                current_pos_y : field::HEIGHT_MAX/2,
                radius: radius,
                color : color,
                x_min: 0,
                x_max: field::WIDTH_MAX/2,
                ///The target position is the same as the current position at initialization
                target_pos_x: field::WIDTH_MAX/4,
                target_pos_y: field::HEIGHT_MAX/2,
                speed_x: 0,
                speed_y: 0,
                input_handler: input_handler,
                graphics_handler: graphics_handler,

            }
        } else {
            Player {
                player_id : player_id,
                current_pos_x : 3* field::WIDTH_MAX/4,
                current_pos_y : field::HEIGHT_MAX/2,
                radius: radius,
                color: color,
                x_min : (field::WIDTH_MAX/2)+1,
                x_max : field::WIDTH_MAX-1,
                ///The target position is the same as the current position at initialization
                target_pos_x: 3* field::WIDTH_MAX/4,
                target_pos_y: field::HEIGHT_MAX/2,
                speed_x: 0,
                speed_y: 0,
                input_handler: input_handler,
                graphics_handler: graphics_handler,
            }
        }
    }
    ///Checks for user input and updates the player accordingly
    ///To be called in the game's main loop
    pub fn update_player(& mut self){
        self.update_on_user_input();
        self.move_player();
        self.draw();
    }

    fn draw(&self){
        self.graphics_handler.draw_player(self.color, [self.current_pos_x, self.current_pos_y], self.radius, false, self.color);
    }
    
    ///Move the player according to the target position
    fn move_player(& mut self){
        //TODO implement delayed movement?
        if helper::unsigned_subtraction(self.current_pos_x, self.target_pos_y) < self.speed_x 
        && helper::unsigned_subtraction(self.current_pos_y, self.target_pos_y) < self.speed_y{
            self.speed_x = 0;
            self.speed_y = 0;
            self.current_pos_x = self.target_pos_x;
            self.current_pos_y = self.target_pos_y;
            
        } else{
            self.current_pos_x += self.speed_x;
            self.current_pos_y += self.speed_y;
        }
    }

    fn update_on_user_input(&self){
        (self.target_pos_x, self.target_pos_y) = input_handler::get_target_position(
            self.current_pos_x, self.current_pos_y, self.radius, self.x_min, self.x_max);
        self.speed_x = helper::unsigned_subtraction(self.target_pos_x, self.current_pos_x)/20;
        self.speed_y = helper::unsigned_subtraction(self.target_pos_y, self.current_pos_y)/20;
    }
    ///Get the player id
    pub fn get_id(&self) -> u8{
        self.player_id
    }
    //Get the current position of the player
    pub fn get_position(&self) -> (u16, u16) {
        (self.current_pos_x, self.current_pos_y)
    }


    pub fn set_position(& mut self, x : u16, y:u16){
        self.current_pos_x = x;
        self.current_pos_y = y;
    }
}

//! Airhockey Player.
use super::helper;
pub const RADIUS: u16 = 10;
pub const COLOR: u32 = 0xfff000;
pub const STROKE_COLOR: u32 = 0xfff000;
pub const HAS_STROKE: bool = false;

use super::{field };
use super::input_handler::{self, InputHandler};
use super::graphics_handler::GraphicsHandler;

use super::init::Handler;

// Player
pub struct Player {
    player_id: u8,
    current_pos: (u16,u16),
    radius:u16,
    color: u32,
    ///Defining the half of the field where the player can move
    x_min: u16,
    x_max: u16,
    ///The player is following the user's input (given by target_position)
    target_pos: (u16,u16),
    ///The speed the player is moving towards the target position
    speed: (u16,u16),
}

impl Player {
    //Create new player

    pub fn new(player_id: u8) -> Player{
        let radius = RADIUS;
        let color = COLOR;
        //This has to be changed if more than two players exist
        if player_id == 0 {
            Player {
                player_id: player_id,
                current_pos : (field::WIDTH_MAX/4,field::HEIGHT_MAX/2),
                radius: radius,
                color : color,
                x_min: 0,
                x_max: field::WIDTH_MAX/2,
                ///The target position is the same as the current position at initialization
                target_pos: (field::WIDTH_MAX/4,field::HEIGHT_MAX/2),
                speed: (0,0),

            }
        } else {
            //TODO zusammenfassen!
            Player {
                player_id : player_id,
                current_pos : (3* field::WIDTH_MAX/4,field::HEIGHT_MAX/2),
                radius: radius,
                color: color,
                x_min : (field::WIDTH_MAX/2)+1,
                x_max : field::WIDTH_MAX-1,
                ///The target position is the same as the current position at initialization
                target_pos: (3* field::WIDTH_MAX/4,field::HEIGHT_MAX/2),
                speed:(0,0),
            }
        }
    }
    
    pub fn draw(&self, handler:&Handler){
        handler.graphics_handler.draw_player(self.color, [self.current_pos.0, self.current_pos.1], self.radius );
    }
    
    ///Move the player according to the target position
    pub fn move_player(& mut self){
        //TODO implement delayed movement?
        if helper::unsigned_subtraction(self.current_pos.0, self.target_pos.1) < self.speed.0 
        && helper::unsigned_subtraction(self.current_pos.1, self.target_pos.1) < self.speed.1{
            self.speed = (0,0);
            self.current_pos = self.target_pos;
            
        } else{
            self.current_pos = (self.current_pos.0 + self.speed.0,self.current_pos.1 + self.speed.1);
        }
    }

    pub fn update_on_user_input(&mut self, handler: &mut Handler){
         if self.player_id == 0{
            handler.input_handler.fetch_input();
        }
        self.target_pos = handler.input_handler.get_target_position(
            self.current_pos, self.radius, self.x_min, self.x_max);
        self.speed = (helper::unsigned_subtraction(self.target_pos.0, self.current_pos.0)/20,
        helper::unsigned_subtraction(self.target_pos.1, self.current_pos.1)/20);
    }
    ///Get the player id
    pub fn get_id(&self) -> u8{
        self.player_id
    }
    //Get the current position of the player
    pub fn get_position(&self) -> (u16, u16) {
        (self.current_pos.0, self.current_pos.1)
    }


    pub fn set_position(& mut self, x : u16, y:u16){
        self.current_pos.0 = x;
        self.current_pos.1 = y;
    }
}

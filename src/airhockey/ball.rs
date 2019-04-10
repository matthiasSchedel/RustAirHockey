//! ball module
/// draw radius of ball
pub const RADIUS: u16 = 10;
/// fill color of ball
pub const COLOR: u32 = 0xfff000;
/// start position of ball in game
pub const START_POSITION: [u16; 2] = [RADIUS, RADIUS];
/// start velocity of ball?
pub const START_SPEED: [f32; 2] = [0.0, 0.0];
/// has bool of ball
pub const STROKE_COLOR: u32 = 0xfff010;
/// has bool stroke?
pub const HAS_STROKE: bool = false;

use super::init::Handler;

/// structrepresents a ball in airhockey game
pub struct Ball {
    pub position: [u16; 2],
    pub speed: [f32; 2],
}
impl Ball {
    /// Ball constructor
    pub fn new() -> Ball {
        Ball {
            position: START_POSITION,
            speed: START_SPEED,
        }
    }

    pub fn draw(&self, handler: &mut Handler) {
        handler.graphics_handler.draw_ball(
            0xff_00_00, /*insert ball color*/
            self.position,
            10, /* insert real radius*/
        )
    }
}

// check ball for colls
// constructs a physics-object from the current game state, checks for collision und updates ball position and speed
// fn check_ball_for_collisons(&mut self, &mut handler: Handler) {
//     handler
//         .physics_handler
//         .physics
//         .set_ball_pos(&self.ball.position[0], &self.ball.position[1]);
//     handler
//         .physics_handler
//         .physics
//         .set_ball_speed(&self.ball.speed[0], &self.ball.speed[1]);
//     let mut active_player: usize = 1;
//     if self.ball.position[0] < (field::WIDTH_MAX / 2) {
//         active_player = 0;
//     }

//     handler.physics_handler.physics.update_ball_position(
//         self.players[active_player].get_position().0,
//         self.players[active_player].get_position().1,
//         /*active_player.radius*/ 10,
//         f32::from(self.players[active_player].get_speed().0),
//         f32::from(self.players[active_player].get_speed().1),
//     );
// }

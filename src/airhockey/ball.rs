//! ball module
/// draw radius of ball
pub const RADIUS:u16 = 10;
/// fill color of ball
pub const COLOR: u32 = 0xfff000; 
/// start position of ball in game
pub const START_POSITION: [u16;2] = [RADIUS,RADIUS];
/// start velocity of ball?
pub const START_SPEED: [f32;2] = [0.0,0.0];
/// has bool of ball
pub const STROKE_COLOR: u32 = 0xfff000; 
/// has bool stroke?
pub const HAS_STROKE: bool = false; 

use super::init::Handler;

/// structrepresents a ball in airhockey game
pub struct Ball {
    position: [u16; 2],
    speed: [f32; 2],

}
impl Ball {
    /// Ball constructor
    pub fn new() -> Ball {
        Ball {
            position: START_POSITION,
            speed: START_SPEED,
        }
    }
}


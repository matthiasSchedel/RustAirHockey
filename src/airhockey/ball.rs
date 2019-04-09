pub const RADIUS:u16 = 10;
pub const COLOR: u32 = 0xfff000; 
pub const START_POSITION: [u16;2] = [RADIUS,RADIUS];
pub const START_SPEED: [f32;2] = [0.0,0.0];
pub const STROKE_COLOR: u32 = 0xfff000; 
pub const HAS_STROKE: bool = false; 


use super::init::Handler;

pub struct Ball {
    position: [u16; 2],
    speed: [f32; 2],

}
impl Ball {
    // Ball constructor
    pub fn new() -> Ball {
        Ball {
            position: START_POSITION,
            speed: START_SPEED,
        }
    }
}


const RADIUS:u16 = 10;
const COLOR: u32 = 0xfff000; 
const START_POSITION: [u16;2] = [RADIUS,RADIUS];
const START_SPEED: [f32;2] = [0.0,0.0];


pub struct Ball {
    pub position: [u16;2],
    pub speed: [f32;2]
}
impl Ball {
    // Ball constructor
    pub fn new() -> Ball {
        Ball {
            position: START_POSITION,
            speed: START_SPEED
        }
    }
}


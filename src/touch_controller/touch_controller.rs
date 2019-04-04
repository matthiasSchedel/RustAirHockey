//! Touch controller.
extern crate alloc;

use alloc::vec::Vec;

pub struct TouchController {
    touch: i32,
    // display width
    width: u16,
    //display height
    height: u16,
}
impl TouchController {
    // game constructor
    pub fn new(width: u16, height: u16) -> TouchController {
        TouchController {
            touch: 2,
            width: width,
            height: height,
        }
    }
    // is touched method
    pub fn is_touched(&self, p_id: usize) -> bool {
        return false;
    }

    pub fn init(&self) {}

    pub fn get_touch_positions(&self) -> (alloc::vec::Vec<u16>, alloc::vec::Vec<u16>) {
        let mut positions_x: Vec<u16> = Vec::new();
        let mut positions_y: Vec<u16> = Vec::new();
        (positions_x, positions_y)
    }
}

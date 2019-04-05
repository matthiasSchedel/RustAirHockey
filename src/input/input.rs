//! Touch controller.
extern crate alloc;

// use crate::i2c::I2C;
use crate::{i2c::I2C, stm32f7::stm32f7x6::I2C3};
use alloc::vec::Vec;

pub struct Input<'a> {
    touch: i32,
    // display width
    width: u16,
    //display height
    height: u16,
    i2c_3: &'a mut I2C<I2C3>,
}
impl<'a> Input<'a> {
    // game constructor
    pub fn new(width: u16, height: u16, i2c_3: &'a mut I2C<I2C3>) -> Input<'a> {
        Input {
            touch: 2,
            width: width,
            height: height,
            i2c_3: i2c_3,
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

//! Touch controller.
extern crate alloc;

// use crate::i2c::I2C;
use crate::{i2c::I2C, stm32f7::stm32f7x6::I2C3};
use alloc::vec::Vec;

/// Input
pub struct Input {
    // display width
    width: u16,
    //display height
    height: u16,
    i2c_3: I2C<I2C3>,
}
impl Input {
    /// Input
    pub fn new(width: u16, height: u16, i2c_3:  I2C<I2C3>) -> Input {
        Input {
            width: width,
            height: height,
            i2c_3: i2c_3,
        }
    }
    /// Input
    pub fn is_touched(&self, p_id: usize) -> bool {
        return false;
    }

    /// Input
    pub fn get_touch_positions(&self) -> (alloc::vec::Vec<u16>, alloc::vec::Vec<u16>) {
        let positions_x: Vec<u16> = Vec::new();
        let positions_y: Vec<u16> = Vec::new();
        (positions_x, positions_y)
    }
}

/// init input
pub fn init(width: u16, height: u16, i2c_3: I2C<I2C3>) -> Input {
    return { Input::new(width, height, i2c_3) };
}

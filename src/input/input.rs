//! Touch controller.
extern crate alloc;

// use crate::i2c::I2C;
use crate::{i2c::I2C, stm32f7::stm32f7x6::I2C3, touch};
use alloc::vec::Vec;

use crate::airhockey::field;

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
    pub fn new(width: u16, height: u16, i2c_3: I2C<I2C3>) -> Input {
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
    pub fn get_touch_positions(&mut self) -> alloc::vec::Vec<(u16, u16)> {
        let mut positions: Vec<(u16, u16)> = Vec::new();
        for touch in touch::touches(&mut self.i2c_3).unwrap() {
            positions.push((touch.x, touch.y));
        }
        return { positions };
    }
        /// check if point is outside
    fn is_out_of_field(&self, point: [u16; 2], pointsize: u16) -> bool {
        let padding: u16 = pointsize + field::BORDER_WIDTH;
        return (self.width - padding > point[0] || point[0] < padding)
            && (self.height - padding > point[1] || point[1] < padding);
    }
}

/// init input
pub fn init(width: u16, height: u16, i2c_3: I2C<I2C3>) -> Input {
    return { Input::new(width, height, i2c_3) };
}

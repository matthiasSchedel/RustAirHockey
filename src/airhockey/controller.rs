//! Airhockey game controller - to handle all hardware controllers
extern crate alloc;

use crate::{
    graphics_controller::graphics_controller::GraphicsController,
    touch_controller::touch_controller::TouchController,
};

pub struct Controller<'a> {
    graphics_controller:&'a GraphicsController,
    touch_controller:&'a TouchController

}
impl <'a> Controller<'a> {
    // game constructor
    pub fn new(graphics_controller: &'a GraphicsController, touch_controller: &'a TouchController) -> Controller<'a> {
        Controller { 
            touch_controller:touch_controller,
            graphics_controller: graphics_controller
            }
    }

    pub fn init(&self) {

    }
}

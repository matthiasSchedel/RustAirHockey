//! Airhockey game controller - to handle all hardware controllers
extern crate alloc;

use crate::{
    graphics_controller::graphics_controller::GraphicsController,
    touch_controller::touch_controller::TouchController,
};

pub struct Controller {
    graphics_controller: GraphicsController,
    touch_controller: TouchController

}
impl Controller {
    // game constructor
    pub fn new(graphics_controller: GraphicsController, touch_controller: TouchController) -> Controller{
        Controller { 
            touch_controller:touch_controller,
            graphics_controller: graphics_controller
            }
    }

    pub fn init(&self) {

    }
}

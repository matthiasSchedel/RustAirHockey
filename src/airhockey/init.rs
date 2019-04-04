// #![feature(alloc)]
// #![feature(alloc_error_handler)]
// #![no_main]
// #![no_std]

// #[macro_use]
// extern crate alloc;
// extern crate alloc_cortex_m;
// extern crate cortex_m;
// extern crate cortex_m_rt as rt;
// extern crate cortex_m_semihosting as sh;
// #[macro_use]
// extern crate stm32f7;
// #[macro_use]
// use stm32f7_discovery;
// extern crate smoltcp;
use crate::{
    graphics_controller::graphics_controller::GraphicsController,
    touch_controller::touch_controller::TouchController,
};
use super::{controller::Controller, game, game::Game, field};
// use crate::{
//     controller::controller::Controller
//     };

// Function init
pub fn init(playerCount: u8) -> Game {
    let controller = createController();
    let game = Game::new(playerCount, controller);
    return game;
}

fn createController() -> Controller {
    let graphics_controller = GraphicsController::new(field::WIDTH_MAX,field::HEIGHT_MAX);
    let touch_controller = TouchController::new(field::WIDTH_MAX,field::HEIGHT_MAX);
    let controller = Controller::new(graphics_controller, touch_controller);
    return controller;
}

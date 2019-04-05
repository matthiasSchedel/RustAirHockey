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
    graphics_controller::graphics_controller::GraphicsController, input::input::Input,
    physics::physics::Physics,
};

use super::{
    field, game, game::Game, graphics_handler, graphics_handler::GraphicsHandler, input_handler,
    input_handler::InputHandler, physics_handler, physics_handler::PhysicsHandler,
};

struct GeneralHardware {}

impl GeneralHardware {
    fn new() -> GeneralHardware {
        GeneralHardware {}
    }
}

struct Handler {
    physics_handler: PhysicsHandler,
    graphics_handler: GraphicsHandler<'static>,
    input_handler: InputHandler<'static>,
}

impl Handler {
    fn new(
        physics_handler: PhysicsHandler,
        graphics_handler: GraphicsHandler<'static>,
        input_handler: InputHandler<'static>,
    ) -> Handler {
        Handler {
            physics_handler: physics_handler,
            graphics_handler: graphics_handler,
            input_handler: input_handler,
        }
    }
}

// use crate::{
//     controller::controller::Controller
//     };

// Function init
pub fn init(playerCount: u8) -> Game {
    // let handler = createHandler();
    let game = Game::new(playerCount);
    return game;
}

fn createHandler() {
    // let handler = Handler::new(physics_handler,graphics_handler,input_handler);
    // return handler;
    //init graphics
    //init physics
    //init input
    // let graphics_handler = GraphicsHandler::new(field::WIDTH_MAX,field::HEIGHT_MAX);
    // let input_handler = Input::new(field::WIDTH_MAX,field::HEIGHT_MAX);
    // let physics_handler = Physics::new(physics_controller, input);
    // return (controller);
}

//init the general hardware
pub fn init_general_hardware() {}

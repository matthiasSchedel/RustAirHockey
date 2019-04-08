#![feature(alloc)]
#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

// use crate::{alloc,alloc_cortex_m, cortex_m, cortex_m_rt as rt, cortex_m_semihosting as sh, stm32f7, smoltcp};

use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout as AllocLayout;
use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m::{asm, interrupt, peripheral::NVIC};
use rt::{entry, exception, ExceptionFrame};
use cortex_m_semihosting::hio::{self, HStdout};
use stm32f7::stm32f7x6::{CorePeripherals, Interrupt, Peripherals};
use smoltcp::{
    dhcp::Dhcpv4Client,
    socket::{
        Socket, SocketSet, TcpSocket, TcpSocketBuffer, UdpPacketMetadata, UdpSocket,
        UdpSocketBuffer,
    },
    time::Instant,
    wire::{EthernetAddress, IpCidr, IpEndpoint, Ipv4Address},
};
use crate::{
    ethernet,
    gpio::{GpioPort, InputPin, OutputPin},
    init,
    lcd::AudioWriter,
    lcd::{self, Color},
    random::Rng,
    sd,
    system_clock::{self, Hz},
    touch,
};

use crate::{
    graphics::graphics::GraphicsController, input::input::Input,
    physics::physics::Physics,
};

use super::{
    field, game, game::Game, graphics_handler, graphics_handler::GraphicsHandler, input_handler,
    input_handler::InputHandler, physics_handler, physics_handler::PhysicsHandler,
};

const HEAP_SIZE: usize = 50 * 1024; // in bytes
const ETH_ADDR: EthernetAddress = EthernetAddress([0x00, 0x08, 0xdc, 0xab, 0xcd, 0xef]);

struct GeneralHardware {

}

impl GeneralHardware {
    fn new() -> GeneralHardware {
        GeneralHardware {
        }
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

/// Function init
pub fn init(playerCount: u8) -> Game {
    // let handler = createHandler();
    let game = Game::new(playerCount);
    return game;
}

pub fn createHandler() {
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






/// init the general hardware
pub fn init_general_hardware<b>() ->((lcd::Layer<lcd::FramebufferArgb8888>, lcd::Layer<lcd::FramebufferAl88>), I2C<I2C3>)  {


    /// initialising LCD screen
    let core_peripherals = CorePeripherals::take().unwrap();
    let mut systick = core_peripherals.SYST;
    let mut nvic = core_peripherals.NVIC;


    let peripherals = Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC;
    let mut pwr = peripherals.PWR;
    let mut flash = peripherals.FLASH;
    let mut fmc = peripherals.FMC;
    let mut ltdc = peripherals.LTDC;
    let mut sai_2 = peripherals.SAI2;
    let mut rng = peripherals.RNG;
    let mut sdmmc = peripherals.SDMMC1;
    let mut syscfg = peripherals.SYSCFG;
    let mut ethernet_mac = peripherals.ETHERNET_MAC;
    let mut ethernet_dma = peripherals.ETHERNET_DMA;


    init::init_systick(Hz(100), &mut systick, &rcc);
    systick.enable_interrupt();


    /// Initialise display port
        let gpio_a = GpioPort::new(peripherals.GPIOA);
    let gpio_b = GpioPort::new(peripherals.GPIOB);
    let gpio_c = GpioPort::new(peripherals.GPIOC);
    let gpio_d = GpioPort::new(peripherals.GPIOD);
    let gpio_e = GpioPort::new(peripherals.GPIOE);
    let gpio_f = GpioPort::new(peripherals.GPIOF);
    let gpio_g = GpioPort::new(peripherals.GPIOG);
    let gpio_h = GpioPort::new(peripherals.GPIOH);
    let gpio_i = GpioPort::new(peripherals.GPIOI);
    let gpio_j = GpioPort::new(peripherals.GPIOJ);
    let gpio_k = GpioPort::new(peripherals.GPIOK);
    let mut pins = init::pins(
        gpio_a, gpio_b, gpio_c, gpio_d, gpio_e, gpio_f, gpio_g, gpio_h, gpio_i, gpio_j, gpio_k,
    );


    init::init_sdram(&mut rcc, &mut fmc);
    let mut lcd = init::init_lcd(&mut ltdc, &mut rcc);
    pins.display_enable.set(true);
    pins.backlight.set(true);

    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();

    layer_1.clear();
    layer_2.clear();
    let mut lcd_display:(lcd::Layer<lcd::FramebufferArgb8888>, lcd::Layer<lcd::FramebufferAl88>) = (layer_1,layer_2);

    // Initialisig touch 
    let mut i2c_3 = init::init_i2c_3(peripherals.I2C3, &mut rcc);
    return (lcd_display, i2c_3);
}
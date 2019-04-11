///! Init airhockey game

use core::mem::uninitialized;
use crate::i2c::I2C;
use crate::{
    gpio::{GpioPort, OutputPin},
    init, lcd::{self,Color},
    system_clock::Hz,
};
use stm32f7::stm32f7x6::I2C3;
use stm32f7::stm32f7x6::{CorePeripherals, Peripherals};

use crate::{graphics::graphics::Graphics, input::input::Input, physics::physics::Physics};

use super::{
    ball, ball::Ball, field, game, game::Game, graphics_handler, graphics_handler::GraphicsHandler,
    input_handler, input_handler::InputHandler, physics_handler, physics_handler::PhysicsHandler,
    player, player::Player, score, score::Score,
};

/// Handler object to access hardware components
pub struct Handler {
    /// struct field
    pub physics_handler: PhysicsHandler,
    /// struct field
    pub graphics_handler: GraphicsHandler,
    /// struct field
    pub input_handler: InputHandler,
}

impl Handler {
    /// handler constructor
    pub fn new(
        physics_handler: PhysicsHandler,
        graphics_handler: GraphicsHandler,
        input_handler: InputHandler,
    ) -> Handler {
        Handler {
            physics_handler: physics_handler,
            graphics_handler: graphics_handler,
            input_handler: input_handler,
        }
    }
}

/// Function init
pub fn init(player_count: u8, handler: Handler) -> Game {
    let game = Game::new(player_count, handler);
    return game;
}
/// create handler function
pub fn create_handler() -> Handler {
    let fps: u16 = 30;
    let hardware: (
        (
            lcd::Layer<lcd::FramebufferArgb8888>,
            lcd::Layer<lcd::FramebufferAl88>,
        ),
        I2C<I2C3>,
    ) = init_general_hardware(fps);
    let layers = ((hardware.0).0, (hardware.0).1);
    let numbers = load_numbers_from_disk();
    let graphics = Graphics::new(field::WIDTH_MAX, field::HEIGHT_MAX, layers, numbers);
    let input = Input::new(field::WIDTH_MAX, field::HEIGHT_MAX, hardware.1);
    let physics = Physics::new(field::WIDTH_MAX, field::HEIGHT_MAX, ball::RADIUS);

    return Handler::new(
        PhysicsHandler::new(physics),
        GraphicsHandler::new(graphics),
        InputHandler::new(input),
    );
}

fn load_numbers_from_disk() -> [lcd::Color;10000]{
    [lcd::Color::rgb(0,0,0);10000]
    
}

 /// Function to lad all values and return reference of array
    pub unsafe fn load_numbers()->[[lcd::Color;10];1000]{
    
        //unpack as array 4 values describe one pixel
        let mut numbers:[[lcd::Color;10];1000];
        numbers = uninitialized();

        // Loading the graphics
        let zero=include_bytes!("./data/0.data");
        let one=include_bytes!("./data/1.data");
        let two=include_bytes!("./data/2.data");
        let three=include_bytes!("./data/3.data");
        let four=include_bytes!("./data/4.data");
        let five=include_bytes!("./data/5.data");
        let six=include_bytes!("./data/6.data");
        let seven=include_bytes!("./data/7.data");
        let eight=include_bytes!("./data/8.data");
        let nine=include_bytes!("./data/9.data");



        for i in 0..one.len(){
            // sorting the list to the rgb fields
            // red
            if i%4==0{
                numbers[0][i/4].red=zero[i];
                numbers[1][i/4].red=one[i];
                numbers[2][i/4].red=two[i];
                numbers[3][i/4].red=three[i];
                numbers[4][i/4].red=four[i];
                numbers[5][i/4].red=five[i];
                numbers[6][i/4].red=six[i];
                numbers[7][i/4].red=seven[i];
                numbers[8][i/4].red=eight[i];
                numbers[9][i/4].red=nine[i];

            }
            //green
            else if i%4==3{
                numbers[0][i/4].green=zero[i];
                numbers[1][i/4].green=one[i];
                numbers[2][i/4].green=two[i];
                numbers[3][i/4].green=three[i];
                numbers[4][i/4].green=four[i];
                numbers[5][i/4].green=five[i];
                numbers[6][i/4].green=six[i];
                numbers[7][i/4].green=seven[i];
                numbers[8][i/4].green=eight[i];
                numbers[9][i/4].green=nine[i];
            }
            //blue
            else if i%4==2{
                numbers[0][i/4].blue=zero[i];
                numbers[1][i/4].blue=one[i];
                numbers[2][i/4].blue=two[i];
                numbers[3][i/4].blue=three[i];
                numbers[4][i/4].blue=four[i];
                numbers[5][i/4].blue=five[i];
                numbers[6][i/4].blue=six[i];
                numbers[7][i/4].blue=seven[i];
                numbers[8][i/4].blue=eight[i];
                numbers[9][i/4].blue=nine[i];
            }
            // alpha
            else {
                numbers[0][i/4].alpha=zero[i];
                numbers[1][i/4].alpha=one[i];
                numbers[2][i/4].alpha=two[i];
                numbers[3][i/4].alpha=three[i];
                numbers[4][i/4].alpha=four[i];
                numbers[5][i/4].alpha=five[i];
                numbers[6][i/4].alpha=six[i];
                numbers[7][i/4].alpha=seven[i];
                numbers[8][i/4].alpha=eight[i];
                numbers[9][i/4].alpha=nine[i];
            }
         }

        numbers
    }


unsafe fn load_end_game()->[[lcd::Color;2];60000]
    {

        let mut player:[[lcd::Color;2];60000];

        player = uninitialized();
        let player_1=include_bytes!("./data/Player1.data");
        let player_2=include_bytes!("./data/Player2.data");
        

        for i in 0..player_1.len(){
                // sorting the list to the rgb fields
                // red
                if i%4==0{
                player[0][i/4].red=player_1[i];
                player[1][i/4].red=player_2[i];

                }
                //green
                else if i%4==3{
                    player[0][i/4].green=player_1[i];
                    player[1][i/4].green=player_2[i];
                }
                //blue
                else if i%4==2{
                    player[0][i/4].blue=player_1[i];
                    player[1][i/4].blue=player_2[i];

                }
                // alpha
                else {
                    player[0][i/4].alpha=player_1[i];
                    player[1][i/4].alpha=player_2[i];
                }
    

            }
        player
}


/// init the general hardware
pub fn init_general_hardware(
    fps: u16,
) -> (
    (
        lcd::Layer<lcd::FramebufferArgb8888>,
        lcd::Layer<lcd::FramebufferAl88>,
    ),
    I2C<I2C3>,
) {
    /// initialising LCD screen
    let core_peripherals = CorePeripherals::take().unwrap();
    let mut systick = core_peripherals.SYST;
    let nvic = core_peripherals.NVIC;

    let peripherals = Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC;
    let mut pwr = peripherals.PWR;
    let mut flash = peripherals.FLASH;
    let mut fmc = peripherals.FMC;
    let mut ltdc = peripherals.LTDC;
    let sai_2 = peripherals.SAI2;
    let rng = peripherals.RNG;
    let sdmmc = peripherals.SDMMC1;
    let syscfg = peripherals.SYSCFG;
    let ethernet_mac = peripherals.ETHERNET_MAC;
    let ethernet_dma = peripherals.ETHERNET_DMA;

    init::init_system_clock_216mhz(&mut rcc, &mut pwr, &mut flash);
    init::enable_gpio_ports(&mut rcc);

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

    init::init_systick(Hz(100), &mut systick, &rcc);
    systick.enable_interrupt();

    init::init_sdram(&mut rcc, &mut fmc);
    let mut lcd = init::init_lcd(&mut ltdc, &mut rcc);
    pins.display_enable.set(true);
    pins.backlight.set(true);

    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();

    layer_1.clear();
    layer_2.clear();
    let lcd_display: (
        lcd::Layer<lcd::FramebufferArgb8888>,
        lcd::Layer<lcd::FramebufferAl88>,
    ) = (layer_1, layer_2);

    // Initialisig touch
    let i2c_3 = init::init_i2c_3(peripherals.I2C3, &mut rcc);
    return (lcd_display, i2c_3);
}

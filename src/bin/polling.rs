#![feature(alloc)]
#![feature(alloc_error_handler)]
#![no_main]
#![no_std]

#[macro_use]
extern crate alloc;
extern crate alloc_cortex_m;
extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
#[macro_use]
extern crate stm32f7;
#[macro_use]
extern crate stm32f7_discovery;
extern crate smoltcp;

use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout as AllocLayout;
use core::fmt::Write;
use core::panic::PanicInfo;
use cortex_m::{asm, interrupt, peripheral::NVIC};
use rt::{entry, exception, ExceptionFrame};
use sh::hio::{self, HStdout};
use smoltcp::{
    dhcp::Dhcpv4Client,
    socket::{
        Socket, SocketSet, TcpSocket, TcpSocketBuffer, UdpPacketMetadata, UdpSocket,
        UdpSocketBuffer,
    },
    time::Instant,
    wire::{EthernetAddress, IpCidr, IpEndpoint, Ipv4Address},
};
use stm32f7::stm32f7x6::{CorePeripherals, Interrupt, Peripherals};
use stm32f7_discovery::{
    airhockey::{self, game::Game},
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

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

const HEAP_SIZE: usize = 50 * 1024; // in bytes
const ETH_ADDR: EthernetAddress = EthernetAddress([0x00, 0x08, 0xdc, 0xab, 0xcd, 0xef]);

#[entry]
fn main() -> ! {
    // let core_peripherals = CorePeripherals::take().unwrap();
    // let mut systick = core_peripherals.SYST;
    // let mut nvic = core_peripherals.NVIC;

    // let peripherals = Peripherals::take().unwrap();
    // let mut rcc = peripherals.RCC;
    // let mut pwr = peripherals.PWR;
    // let mut flash = peripherals.FLASH;
    // let mut fmc = peripherals.FMC;
    // let mut ltdc = peripherals.LTDC;
    // let mut sai_2 = peripherals.SAI2;
    // let mut rng = peripherals.RNG;
    // let mut sdmmc = peripherals.SDMMC1;
    // let mut syscfg = peripherals.SYSCFG;
    // let mut ethernet_mac = peripherals.ETHERNET_MAC;
    // let mut ethernet_dma = peripherals.ETHERNET_DMA;

    // init::init_system_clock_216mhz(&mut rcc, &mut pwr, &mut flash);
    // init::enable_gpio_ports(&mut rcc);

    // let gpio_a = GpioPort::new(peripherals.GPIOA);
    // let gpio_b = GpioPort::new(peripherals.GPIOB);
    // let gpio_c = GpioPort::new(peripherals.GPIOC);
    // let gpio_d = GpioPort::new(peripherals.GPIOD);
    // let gpio_e = GpioPort::new(peripherals.GPIOE);
    // let gpio_f = GpioPort::new(peripherals.GPIOF);
    // let gpio_g = GpioPort::new(peripherals.GPIOG);
    // let gpio_h = GpioPort::new(peripherals.GPIOH);
    // let gpio_i = GpioPort::new(peripherals.GPIOI);
    // let gpio_j = GpioPort::new(peripherals.GPIOJ);
    // let gpio_k = GpioPort::new(peripherals.GPIOK);
    // let mut pins = init::pins(
    //     gpio_a, gpio_b, gpio_c, gpio_d, gpio_e, gpio_f, gpio_g, gpio_h, gpio_i, gpio_j, gpio_k,
    // );

    // // configures the system timer to trigger a SysTick exception every second
    // init::init_systick(Hz(100), &mut systick, &rcc);
    // systick.enable_interrupt();

    // init::init_sdram(&mut rcc, &mut fmc);
    // let mut lcd = init::init_lcd(&mut ltdc, &mut rcc);
    // pins.display_enable.set(true);
    // pins.backlight.set(true);

    // let mut layer_1 = lcd.layer_1().unwrap();
    // let mut layer_2 = lcd.layer_2().unwrap();

    // layer_1.clear();
    // layer_2.clear();
    // lcd::init_stdout(layer_2);

    // println!("Hello World");

    // // Initialize the allocator BEFORE you use it
    // unsafe { ALLOCATOR.init(rt::heap_start() as usize, HEAP_SIZE) }

    // let _xs = vec![1, 2, 3];

    // let mut i2c_3 = init::init_i2c_3(peripherals.I2C3, &mut rcc);
    // i2c_3.test_1();
    // i2c_3.test_2();

    // nvic.enable(Interrupt::EXTI0);

    // let mut sd = sd::Sd::new(&mut sdmmc, &mut rcc, &pins.sdcard_present);

    // init::init_sai_2(&mut sai_2, &mut rcc);
    // init::init_wm8994(&mut i2c_3).expect("WM8994 init failed");
    // // touch initialization should be done after audio initialization, because the touch
    // // controller might not be ready yet
    // touch::check_family_id(&mut i2c_3).unwrap();

    // let mut rng = Rng::init(&mut rng, &mut rcc).expect("RNG init failed");

    // let mut previous_button_state = pins.button.get();
    // let mut audio_writer = AudioWriter::new();


    //init airhockey game with 2 players
    let handler = airhockey::init::createHandler();
    
    let airhockey_game = airhockey::init::init(2, &handler);
    airhockey_game.start(10, 5, true);
    // start the game loop
    airhockey_game.game_loop();

    // let graphics =

    // airhockey_game.init();

    // loop {
        // handle input
        //      if touchcontroller().rightPlayerTouched == true
        // handle collision
        //          game.move(rightPlayer)
        // draw
        //      graphicController.draw()
        //

        // poll button state
        // let current_button_state = pins.button.get();
        // if current_button_state != previous_button_state {
        //     if current_button_state {
        //         pins.led.toggle();

        //         // trigger the `EXTI0` interrupt
        //         NVIC::pend(Interrupt::EXTI0);
        //     }

        //     previous_button_state = current_button_state;
        // }

        // let width_max = 523;
        // let heigth_max = 293;

        // // poll for new touch data
        // for touch in &touch::touches(&mut i2c_3).unwrap() {
        //     for i in (0..10) {
        //         for j in (0..10) {
        //             if (touch.x + i - 5) < width_max && (touch.y - 5 + j) < heigth_max {
        //                 let tx: usize = (touch.x as usize) + (i as usize) - 5;
        //                 let ty: usize = (touch.y as usize) + (j as usize) - 5;
        //                 layer_1.print_point_color_at(
        //                     tx as usize,
        //                     ty as usize,
        //                     Color::from_hex(0xffff00),
        //                 );
        //             }
        //         }
        //     }
        //     layer_1.print_point_color_at(
        //         touch.x as usize,
        //         touch.y as usize,
        //         Color::from_hex(0xffff00),
        //     );
        //     // println!("{}", touch.x);
        //     // println!("{}", touch.y);
        // }

        // // Initialize the SD Card on insert and deinitialize on extract.
        // if sd.card_present() && !sd.card_initialized() {
        //     if let Some(i_err) = sd::init(&mut sd).err() {
        //         println!("{:?}", i_err);
        //     }
        // } else if !sd.card_present() && sd.card_initialized() {
        //     sd::de_init(&mut sd);
        // }
    // }
}

fn poll_socket(socket: &mut Socket) -> Result<(), smoltcp::Error> {
    match socket {
        &mut Socket::Udp(ref mut socket) => match socket.endpoint().port {
            15 => loop {
                let reply;
                match socket.recv() {
                    Ok((data, remote_endpoint)) => {
                        let mut data = Vec::from(data);
                        let len = data.len() - 1;
                        data[..len].reverse();
                        reply = (data, remote_endpoint);
                    }
                    Err(smoltcp::Error::Exhausted) => break,
                    Err(err) => return Err(err),
                }
                socket.send_slice(&reply.0, reply.1)?;
            },
            _ => {}
        },
        &mut Socket::Tcp(ref mut socket) => match socket.local_endpoint().port {
            15 => {
                if !socket.may_recv() {
                    return Ok(());
                }
                let reply = socket.recv(|data| {
                    if data.len() > 0 {
                        let mut reply = Vec::from("tcp: ");
                        let start_index = reply.len();
                        reply.extend_from_slice(data);
                        reply[start_index..(start_index + data.len() - 1)].reverse();
                        (data.len(), Some(reply))
                    } else {
                        (data.len(), None)
                    }
                })?;
                if let Some(reply) = reply {
                    assert_eq!(socket.send_slice(&reply)?, reply.len());
                }
            }
            _ => {}
        },
        _ => {}
    }
    Ok(())
}

interrupt!(EXTI0, exti0, state: Option<HStdout> = None);

fn exti0(_state: &mut Option<HStdout>) {
    println!("Interrupt fired! This means that the button was pressed.");
}

#[exception]
fn SysTick() {
    system_clock::tick();
    // print a `.` every 500ms
    if system_clock::ticks() % 50 == 0 && lcd::stdout::is_initialized() {
        print!(".");
    }
}

#[exception]
fn HardFault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn rust_oom(_: AllocLayout) -> ! {
    panic!("out of memory");
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    interrupt::disable();

    if lcd::stdout::is_initialized() {
        println!("{}", info);
    }

    if let Ok(mut hstdout) = hio::hstdout() {
        let _ = writeln!(hstdout, "{}", info);
    }

    // OK to fire a breakpoint here because we know the microcontroller is connected to a debugger
    asm::bkpt();

    loop {}
}

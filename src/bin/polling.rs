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
use smoltcp::{socket::Socket, time::Instant, wire::EthernetAddress};
use stm32f7::stm32f7x6::{CorePeripherals, Interrupt, Peripherals};
use stm32f7_discovery::{
    airhockey,
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
                                    // const ETH_ADDR: EthernetAddress = EthernetAddress([0x00, 0x08, 0xdc, 0xab, 0xcd, 0xef]);

#[entry]
fn main() -> ! {
    let airhockey_game = airhockey::init::init(2);
    airhockey_game.start(10, 5, true);
    loop {
        while airhockey_game.game_loop() {
            
        }
    }
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

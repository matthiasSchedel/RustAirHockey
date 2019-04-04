// ich möchte mit cargo run --example display --release gebaut werden, weil Graphik und Debug nicht cool ist.
#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc_cortex_m;
extern crate cortex_m;
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting;
extern crate stm32f7;
extern crate stm32f7_discovery;

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout as AllocLayout;
use core::panic::PanicInfo;
use rt::{entry, exception};
use stm32f7::stm32f7x6::{Peripherals};
use stm32f7_discovery::{
    gpio::{GpioPort, OutputPin},
    init,
    system_clock::{self},
    lcd::{self, Color},
    
};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC;
    let mut flash = peripherals.FLASH;
    let mut pwr = peripherals.PWR;
    let mut ltdc = peripherals.LTDC;
    let mut fmc = peripherals.FMC;


    init::init_system_clock_216mhz(&mut rcc, &mut pwr, &mut flash);
    init::enable_gpio_ports(&mut rcc);

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
    
    //fuers Display brauchen wir den sdram und das lcd
    init::init_sdram(&mut rcc, &mut fmc);
    let mut lcd = lcd::init(&mut ltdc, &mut rcc);
    
    // das display hat 2 Layer auf die wir zeichnen können
    let mut layer1 = lcd::Lcd::layer_1(&mut lcd).unwrap();
    let mut layer2 = lcd::Lcd::layer_2(&mut lcd).unwrap();
    
    // dass wir was sehen, machen wir das Display und das Licht an
    pins.display_enable.set(true);
    pins.backlight.set(true);

    // alte Sachen wegraeumen
    layer1.clear();
    layer2.clear();

    // ab hier fancy Dinge, die nicht schön implmentiert sind...
    let mut velocity_x: i32 = 3;
    let mut velocity_y: i32 = 1;

    let border_width = 5;


    // hier entsteht das "Spielfeld" - hier nur ein Rahmen mit Öffnungen am Rand.
    for x in 0..480 {
        for y in 0..border_width {
            layer2.print_point_color_at(x, y, Color::from_hex(0x000_000));
        }

        for y in 272-border_width..272 {
            layer2.print_point_color_at(x,y, Color::from_hex(0x000_000));
        }
    }

    for y in 0..101 {
        for x in 0..border_width {
                layer2.print_point_color_at(x, y, Color::from_hex(0x000_000));
        }
        for x in 480-border_width..480 {
                layer2.print_point_color_at(x, y, Color::from_hex(0x000_000));
        }
    }

    for y in 171..272 {
        for x in 0..border_width {
                layer2.print_point_color_at(x, y, Color::from_hex(0x000_000));
        }
        for x in 480-border_width..480 {
                layer2.print_point_color_at(x, y, Color::from_hex(0x000_000));
        }
    }

    // hier sind die Größen, die vom Display übrig bleiben, wenn wir den Rand abziehen
    let limit_x:i32 = 475;
    let limit_y:i32 = 267; 

    fn in_circle(x: usize, y: usize, center_x: usize, center_y: usize, radius: usize) -> bool {
        (x-center_x)*(x-center_x) + (y-center_y)*(y-center_y) <= radius*radius
    }

    let radius = 10;
    let mut center_x = 170;
    let mut center_y = 100;

    // und das ist der Main Loop
    loop {
        //und jetzt können wir alles machen, was wir auf Layern machen können.
        
        // in jeder Iteration wird das ganze Feld neu gezeichnet
        for x in 0..480 {
            for y in 0..272 {
                if in_circle(x, y, center_x, center_y, radius) {
                    //entweder rot, wenn der Kreis grade hier ist
                    layer1.print_point_color_at(x as usize, y as usize, Color::from_hex(0xff0_000));
                } else {
                    //oder blau, wenn hier grade nichts ist.
                    layer1.print_point_color_at(x as usize,y as usize, Color::from_argb8888(0));
                }
            }
        }


        // hier errechnet sich die neue Position. Wenn keine Verschiebung möglich ist, bleibt es stehen und ändert die Richtung.
        // das ist bedingt unpraktisch, wenn die Geschwindigkeit größer 1 ist, dreht das schon vor der Kollision.
        if center_x + radius + velocity_x as usize <= limit_x as usize && center_x - radius as usize + velocity_x as usize >= 5 {
            center_x += velocity_x as usize;
        } else {
            velocity_x *= -1;
        }

        if center_y + radius + velocity_y as usize <= limit_y as usize && center_y - radius as usize + velocity_y as usize >= 5 {
            center_y += velocity_y as usize;
        } else {
            velocity_y *= -1;
        }

    }
}

#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[exception]
fn SysTick() {
    system_clock::tick();
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn rust_oom(_: AllocLayout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;
    use cortex_m::asm;
    use cortex_m_semihosting::hio;

    if let Ok(mut hstdout) = hio::hstdout() {
        let _ = writeln!(hstdout, "{}", info);
    }

    // OK to fire a breakpoint here because we know the microcontroller is connected to a debugger
    asm::bkpt();

    loop {}
}

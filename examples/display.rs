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
extern crate libm;

use alloc_cortex_m::CortexMHeap;
use libm::F32Ext;
use core::alloc::Layout as AllocLayout;
use core::panic::PanicInfo;
use rt::{entry, exception};
use stm32f7::stm32f7x6::Peripherals;
use stm32f7_discovery::{
    gpio::{GpioPort, OutputPin},
    init,
    lcd::{self, Color, FramebufferArgb8888},
    system_clock, touch,
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

    let mut i2c_3 = init::init_i2c_3(peripherals.I2C3, &mut rcc);
    i2c_3.test_1();
    i2c_3.test_2();

    // poll for new touch data

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

    fn circle_collision(center1_x: usize, center1_y: usize, radius1: usize, center2_x: usize, center2_y: usize, radius2: usize) -> bool{
        // Kreise kollidieren dann, wenn der Abstand zwischen den Mittelpunkten kleiner ist als die Summer der Radien
        // Wurzeln vermeiden, dementsprechend ist alles hier quadriert.
        let distance = (center1_x-center2_x)*(center1_x-center2_x) + (center1_y-center2_y)*(center1_y-center2_y);
        let min_distance = (radius1 + radius2) * (radius1 + radius2);
        distance < min_distance
    }

    // und das ist der Main Loop
    loop {
        //und jetzt können wir alles machen, was wir auf Layern machen können.
        let length = 20;
        let hight = 10;
        let HIGHT = 272;
        let LENGTH = 480;
        let mut max_length;
        let mut max_hight;
        let mut old_touchx: usize;
        let mut old_touchy: usize;

        for touch in &touch::touches(&mut i2c_3).unwrap() {
            // Check if rectangle is bigger then screen
            if touch.x as usize + length > LENGTH {
                max_length = LENGTH;
            } else {
                max_length = touch.x as usize + length;
            }

            if touch.y as usize + hight > HIGHT {
                max_hight = HIGHT;
            } else {
                max_hight = touch.y as usize + hight;
            }
            //draw_rectangle(&mut layer1, touch.x as usize, touch.y as usize, max_length, max_hight, Color::from_hex(0xff0_000))
            draw_circle(
                &mut layer1,
                touch.x as usize,
                touch.y as usize,
                20,
                Color::from_hex(0xff0_000),
            )
        }

        // und hier gibt es die Kollision zwischen Kugeln
        if circle_collision(center_x, center_y, radius, center_x2, center_y2, radius2) {
            // Bingo. Hier magische Physik einfügen. Massenerhaltung, Energieerhaltung, Impulserhaltung
            // das jetzt eher ein Monster, aber ich mag es.

            // wir brauchen: die Normalenvektoren
            let mut norm_x: i16 = center_x as i16- center_x2 as i16;
            let mut norm_y: i16 = center_y as i16- center_y2 as i16;

            let distsquare: f32 = (norm_x*norm_x + norm_y *norm_y).into();
            let dist = distsquare.sqrt();

            // wir normalisieren das
            norm_x = (f32::from(norm_x) / dist) as i16;
            norm_y /= dist as i16;

            // dann noch einen Tangentialvektor
            let tan_x = norm_y;
            let tan_y = -norm_x;

            // die Überlappung
            let overlap = radius + radius2 - dist as usize;

            //wir schieben das etwas auseinander, um micht mehrmals zu kollidieren
            center_x = center_x - norm_x as usize* overlap as usize;
            center_y = center_y - norm_y as usize * overlap as usize;

            center_x2 = center_x2 - norm_x as usize * overlap as usize;
            center_y2 = center_y2 - norm_y as usize* overlap as usize;

            // die Geschwindigeiten wollen in normal- und Tangentialanteile zerlegt
            // erster Kreis
            let velocity_norm = norm_x * velocity_x as i16 + norm_y * velocity_y as i16;
            let velocity_tan = tan_x * velocity_x as i16 + tan_y * velocity_y as i16;

            let share_norm_x = norm_x * velocity_norm; // norm-vektor-anteile von der Geschwindigeit -> x
            let share_norm_y = norm_y * velocity_norm; // norm-vektor-anteile von der Geschwindigkeit -> y

            let share_tan_x = tan_x * velocity_tan; // tangentialanteile von der Geschwindigkeit -> x
            let share_tan_y = tan_y * velocity_tan; // tangentialanteile von der Geschwindigeit -> y

            // zweiter Kreis
            let velocity_norm2 = norm_x * velocity_x2 as i16 + norm_y * velocity_y2 as i16;
            let velocity_tan2 = tan_x as i16 * velocity_x2 as i16 + tan_y as i16* velocity_y2 as i16;

            let share_norm_x2 = norm_x * velocity_norm2;
            let share_norm_y2 = norm_y * velocity_norm2;

            let share_tan_x2 = tan_x as i16 * velocity_tan2;
            let share_tan_y2 = tan_y as i16 * velocity_tan2;

            // in 2D bleiben die Tangentialateile immer erhalten, kollidiiert wird entlang der Normalen.
            // das war mal Impulserhaltung, ich habe aber alle Massen = 1 gesetzt.
            let pulse_x = share_norm_x + share_norm_x2;
            let pulse_y = share_norm_y + share_norm_y2;

            velocity_x = pulse_x as i32- share_norm_x as i32 + share_tan_x as i32;
            velocity_y = pulse_y as i32- share_norm_y as i32 + share_tan_y as i32;

            velocity_x2 = pulse_x as i32 - share_norm_x2 as i32+ share_tan_x2 as i32;
            velocity_y2 = pulse_y as i32- share_norm_y2 as i32 + share_tan_y2 as i32;







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

// Noting to see here

pub fn draw_circle(
    layer: &mut lcd::Layer<FramebufferArgb8888>,
    x_centre: usize,
    y_centre: usize,
    radius: usize,
    color: lcd::Color,
) {
    for y in y_centre - radius..=y_centre + radius {
        for x in x_centre - radius..=x_centre + radius {
            if x * x + x_centre * x_centre - 2 * x * x_centre + y * y + y_centre * y_centre
                - 2 * y * y_centre
                <= radius * radius
            {
                layer.print_point_color_at(x as usize, y as usize, color);
            }
        }
    }
}

pub fn draw_rectangle(
    layer: &mut lcd::Layer<FramebufferArgb8888>,
    x_start: usize,
    y_start: usize,
    length: usize,
    hight: usize,
    color: lcd::Color,
) {
    for x in x_start..=length {
        for y in y_start..=hight {
            layer.print_point_color_at(x as usize, y as usize, color);
        }
    }
}

//! Graphics controller.



use crate::{
    gpio::{GpioPort, OutputPin},
    init,
    lcd::{self, Color, FramebufferArgb8888},
    system_clock, touch,
};
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout as AllocLayout;
use core::panic::PanicInfo;
use rt::{entry, exception};
use stm32f7::stm32f7x6::Peripherals;

const STROKE_COLOR: u32 = 0xffff00;
const USE_STROKE: bool = true;
const PLAYER_SIZE: u16 = 10;
const PUCK_SIZE: u16 = 6;
const BACKGROUND_COLOR: u32 = 0xfff000;

/// Graphics struct
pub struct Graphics {
    /// display layer
    display_layer: (
        lcd::Layer<lcd::FramebufferArgb8888>,
        lcd::Layer<lcd::FramebufferAl88>,
    ),
    /// display width
    width: u16,
    ///display height
    height: u16,
}
impl Graphics {
    /// game constructor
    pub fn new(
        width: u16,
        height: u16,
        display_layer: (
            lcd::Layer<lcd::FramebufferArgb8888>,
            lcd::Layer<lcd::FramebufferAl88>,
        ),
    ) -> Graphics {
        Graphics {
            display_layer: display_layer,
            width: width,
            height: height,
        }
    }
    /// is touched method
    pub fn is_touched(&self, p_id: usize) -> bool {
        return false;
    }

    ///draw a circle around pos x,y with radius - and
    pub fn draw_circle(
        &mut self,
        color: u32,
        pos: [u16; 2],
        radius: u16,
        draw_stroke: bool,
        stroke_color: u32,
    ) {
        let mut x_test = 0;
        let pos_x = usize::from(pos[0]);
        let pos_y = usize::from(pos[1]);
        assert!(pos_x < 523);
        assert!(pos_y < 293);

        for y in pos_y - usize::from(radius)..=pos_y + usize::from(radius) {
            for x in usize::from(pos[0] - radius)..=usize::from(pos[0] + radius) {
                x_test =
                    x * x + y * y + pos_y * pos_y - 2 * y * pos_y + pos_x * pos_x - 2 * x * pos_x;
                if x_test <= usize::from(radius) * usize::from(radius) {
                    self.display_layer
                        .1
                        .print_point_color_at(x, y, Color::from_hex(color));
                }
            }
        }
    }
    ///  clear a circle
    pub fn clear_circle(&self, color: u16, pos: [u16; 2], radius: f32) {}

    ///  clear the field
    pub fn clear_field(&self, color: u16) {}

    /// draw a score
    pub fn draw_score(&self, player1_score: u8, player2_score: u8) {}

    /// init
    pub fn init(&self) {}

    /// draw a rectangle
    pub fn draw_rectangle(
        &mut self,
        /*layer: &mut lcd::Layer<FramebufferArgb8888>,*/
        x_start: u16,
        y_start: u16,
        x_end: u16,
        y_end: u16,
        color: u32,
    ) {
        for x in x_start..x_end {
            for y in y_start..y_end {
                self.display_layer.0.print_point_color_at(
                    x as usize,
                    y as usize,
                    lcd::Color::from_hex(color),
                );
            }
        }
    }
    ///method for drawing the field
    pub fn draw_field(
        &mut self,
        color: u32,
        field_size: [u16; 2],
        border_width: u16,
        goal_size: u16,
    ) {
        // lower rectangle
        self.draw_rectangle(0, 0, field_size[0], border_width, color);

        // upper rectangle
        self.draw_rectangle(
            0,
            field_size[1] - border_width,
            field_size[0],
            field_size[1],
            color,
        );

        // left side
        self.draw_rectangle(0, 0, border_width, (field_size[1] - goal_size) / 2, color);
        self.draw_rectangle(
            0,
            (field_size[1] + goal_size) / 2,
            border_width,
            field_size[1],
            color,
        );

        // draw right side
        self.draw_rectangle(
            field_size[0] - border_width,
            0,
            field_size[0],
            (field_size[1] - goal_size) / 2,
            color,
        );
        self.draw_rectangle(
            field_size[0] - border_width,
            (field_size[1] + goal_size) / 2,
            field_size[0],
            field_size[1],
            color,
        );
    }

    pub fn draw_numbers(
        &mut self,
        number_array:[[lcd::Color;25];40],
        position:[u16;2],
        ){
            for x in position[0] as usize..position[0] as usize{
                for y in position[1].. position[1]+2{
                     self.display_layer.0.print_point_color_at(
                    x as usize,
                    y as usize,
                    number_array[x as usize ][y as usize]);
                }
            }




    }

    pub fn get_numbers(){

        //define pixel size
        let number_width:usize=25;
        let number_hight:usize=40;

        // Loading the graphics
        let one=include_bytes!("1.data");
        let two=include_bytes!("2.data");
        let three=include_bytes!("3.data");
        let four=include_bytes!("4.data");
        let five=include_bytes!("5.data");
        let six=include_bytes!("6.data");
        let seven=include_bytes!("7.data");
        let eight=include_bytes!("8.data");
        let nine=include_bytes!("9.data");

        //unpack as array 4 values describe one pixel
        let number:[lcd::Color;1000];
        let array_number:[[lcd::Color;25];40];


        
        for i in 0..one.len(){
            // sorting the list to the rgb fields
            // red
            if i%4==0{
                number[i/4].red=one[i];
            }
            //green
            else if i%4==3{
                number[i/4].green=one[i];
            }
            //blue
            else if i%4==2{
                number[i/4].blue=one[i];
            }
            // alpha
            else {
                number[i/4].alpha=one[i];
            }

            // merging to right array
            array_number[i/number_width][i%number_width]=number[i];
         }


    }
}

// /// function for drawing the basic field
// pub fn draw_field(
//     layer: &mut lcd::Layer<FramebufferArgb8888>,
//     color: lcd::Color,
// ){
//     // import global size of filed
//     let HEIGHT=airhockey::field::HEIGHT_MAX;
//     let WIDTH=airhockey::field::WIDTH_MAX;;
//     // define width of field
//     let width=10;
//     // define goalsize
//     let goal_size=50;

//     // lower rectangle
//     draw_rectangle(layer, 0 , 0 , WIDTH  , width  , color);

// /// function for random initializing the ball
// pub fn initialize_ball_poisition(
//     layer: &mut lcd::Layer<FramebufferArgb8888>,
//     color: lcd::Color,

// ){
//     let x_position=random_int_generatror(200,250);
//     let y_position=random_int_generatror(100,150);
//     draw_circle(layer, x_position as u16, y_position as u16, 10,color);

// }

// pub fn random_int_generatror(
//     // Uses toml
//     // use rand::Rng;
//     // use rand::SeedableRng;
//     x_bound_low:u16,
//     x_bound_high:u16,
// )-> u16{
//     let mut rand= rand::rngs::StdRng::seed_from_u64(54531212);
//      let rdm_x=rand.gen_range(x_bound_low,x_bound_high);
//      rdm_x as u16
// }

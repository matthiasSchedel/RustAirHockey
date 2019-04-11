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
const NUMBER_HIGHT:u16=40;
const NUMBER_WIDTH:u16=25;
const DOUBLE_DOT_COLOR:u32=0xffff00;

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
    pub fn draw_score(
        &self, 
        player1_score: u8, 
        player2_score: u8,
        number_array:[lcd::Color;1000],
        ) {
        // Draws two number and : 2:1

        // First number
        self.draw_number(number_array,[225,15]);
        // Second number
        self.draw_number(number_array,[265,15]);

        // Double dot
        self.draw_circle(self::DOUBLE_DOT_COLOR, [240,25],2,false, self::DOUBLE_DOT_COLOR);
        self.draw_circle(self::DOUBLE_DOT_COLOR, [240,35],2,false, self::DOUBLE_DOT_COLOR);
    }

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
    /// draw numbers on screen at positi
    pub fn draw_number(
        &mut self,
        // array with the information of number
        number_array:[lcd::Color;1000],
        // upper left position of number rectangle
        position:[u16;2],
        // Dimension of number rectangle
        ){
            let mut x_pos:u16;
            let mut y_pos:u16;
            for i in 0..1000{
                // unpack 1darray to 2darray
                x_pos=i%self::NUMBER_WIDTH;
                y_pos=i/self::NUMBER_HIGHT;

                 self.display_layer.0.print_point_color_at(
                     x_pos as usize + position[0] as usize,
                    y_pos as usize+ position[1] as usize,
                    number_array[i as usize]
                );
                

            }

            }


//     pub fn load_numbers(){

//         //define pixel size
//         let number_width:usize=25;
//         let number_hight:usize=40;

//         // Loading the graphics
//         let zero=include_bytes!("0.data");
//         let one=include_bytes!("1.data");
//         let two=include_bytes!("2.data");
//         let three=include_bytes!("3.data");
//         let four=include_bytes!("4.data");
//         let five=include_bytes!("5.data");
//         let six=include_bytes!("6.data");
//         let seven=include_bytes!("7.data");
//         let eight=include_bytes!("8.data");
//         let nine=include_bytes!("9.data");

//         //unpack as array 4 values describe one pixel
//         let number_one:[lcd::Color;1000];

//         for i in 0..one.len(){
//             // sorting the list to the rgb fields
//             // red
//             if i%4==0{
//                 number_one[i/4].red=one[i];
//             }
//             //green
//             else if i%4==3{
//                 number_one[i/4].green=one[i];
//             }
//             //blue
//             else if i%4==2{
//                 number_one[i/4].blue=one[i];
//             }
//             // alpha
//             else {
//                 number_one[i/4].alpha=one[i];
//             }

//          }


//     }
// }


    /// Function to lad all values and return reference of array
    pub fn load_numbers()->&'static[[lcd::Color;10];1000]{

        //define pixel size
        let number_width:usize=25;
        let number_hight:usize=40;

        // Loading the graphics
        let zero=include_bytes!("0.data");
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
        let mut numbers:[[lcd::Color;10];1000];

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
            //blue
            else if i%4==2{
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
            // alpha
            else {
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
         }

        &numbers
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
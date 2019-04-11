//! Graphics controller.



use core::mem::uninitialized;
use crate::{
    gpio::{GpioPort, OutputPin},
    init,
    lcd::{self, Color, FramebufferArgb8888},
    system_clock, touch, alloc
};
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout as AllocLayout;
use core::panic::PanicInfo;
use alloc::vec::Vec;
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
const END_WIDTH:u16=300;
const END_HIGHT:u16=200;

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

    /* /// check if point is outside
    fn isPointOutside(&self, point: [u16; 2], pointsize: u16) -> bool {
        return (self.width > point[0] + pointsize || point[0] - pointsize < 0)
            && (self.height > point[1] + pointsize || point[1] > 0);
    } */

    ///draw a circle around pos x,y with radius
    pub fn draw_circle(
        &mut self,
        color: u32,
        pos: [u16; 2],
        radius: u16,
        draw_stroke: bool,
        stroke_color: u32,
    ) {
        let mut x_test = 0;
        assert!(pos[0] < self.width);
        assert!(pos[1] < self.height);
        let pos_x = usize::from(pos[0]);
        let pos_y = usize::from(pos[1]);

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
    ///Clear the specified layer
    pub fn clear_layer (&mut self, layer:u8){
        assert!(layer == 0 || layer == 1);
        if layer == 0 {
        self.display_layer.0.clear();
        } else {
            self.display_layer.1.clear()
        }
    }
    ///  clear a circle
    pub fn clear_circle(&self, color: u16, pos: [u16; 2], radius: f32) {}

    ///  clear the field
    pub fn clear_field(&self, color: u16) {}

    /// draw a score
    pub fn draw_score(
        &mut self, 
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

        // draw middle line
        self.draw_rectangle(
            field_size[0]/2 - border_width/4,
            0,
            field_size[0]/2 + border_width/4,
            field_size[1],
            color,
        );
    }

    pub fn draw_circles_implicit(&mut self, circles: &Vec<((u16,u16), u16, u32)>) {

        for x in 0..self.width {
            for y in 0..self.height {
                let mut in_circle = false;
                for circle in circles {
                    if self.in_circle(x, y, [(circle.0).0, (circle.0).1], circle.1) {
                        (self.display_layer.1).print_point_color_at(x as usize, y as usize, Color::from_hex(circle.2));
                        in_circle = true;
                    }
                }
                if !in_circle {
                    (self.display_layer.1).print_point_color_at(x as usize, y as usize, Color::from_argb8888(0));
                }
            }
        }
    }

    fn in_circle(&self, x: u16, y: u16, pos: [u16; 2], radius: u16) -> bool {
        (i32::from(x)  - i32::from(pos[0]))*(i32::from(x) - i32::from(pos[0])) + (i32::from(y)-i32::from(pos[1]))*(i32::from(y)-i32::from(pos[1])) <= i32::from(radius)*i32::from(radius)
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

    pub fn draw_endgame(
         &mut self,
        // array with the information of number
        number_array:[lcd::Color;60000],
        // upper left position of number rectangle
        position:[u16;2],
        // Dimension of number rectangle

    ){
        let mut x_pos:u16;
        let mut y_pos:u16;
        for i in 0..6000{
            x_pos=i%self::END_WIDTH;
            y_pos=i/self::END_HIGHT;
            self.display_layer.0.print_point_color_at(
                    x_pos as usize + position[0] as usize,
                    y_pos as usize+ position[1] as usize,
                    number_array[i as usize]
            );

        }


        }
 




    /// Function to lad all values and return reference of array
    pub unsafe fn load_numbers()->[[lcd::Color;10];1000]{
    
        //unpack as array 4 values describe one pixel
        let mut numbers:[[lcd::Color;10];1000];
        numbers = uninitialized();

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


pub unsafe fn load_end_game()->[[lcd::Color;2];60000]
    {

        let mut player:[[lcd::Color;2];60000];

        player = uninitialized();
        let player_1=include_bytes!("Player1.data");
        let player_2=include_bytes!("Player2.data");
        

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
}


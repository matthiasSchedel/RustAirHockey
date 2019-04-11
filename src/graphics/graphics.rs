//! Graphics controller.

use crate::{
    alloc,
    gpio::{GpioPort, OutputPin},
    init,
    lcd::{self, Color, FramebufferArgb8888},
    system_clock, touch,
};
use alloc::vec::Vec;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout as AllocLayout;
use core::mem::uninitialized;
use core::panic::PanicInfo;
use rt::{entry, exception};
use stm32f7::stm32f7x6::Peripherals;

const STROKE_COLOR: u32 = 0xffff00;
const USE_STROKE: bool = true;
const PLAYER_SIZE: u16 = 10;
const PUCK_SIZE: u16 = 6;
const BACKGROUND_COLOR: u32 = 0xfff000;
const NUMBER_HIGHT: u16 = 40;
const NUMBER_WIDTH: u16 = 25;
const DOUBLE_DOT_COLOR: u32 = 0x000000;
const END_WIDTH: u16 = 300;
const END_HIGHT: u16 = 200;

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
    numbers:[&'static [u8;4000]; 10],
    game_over_images:[&'static [u8;320000]; 2],
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
       numbers:[&'static [u8;4000]; 10],
        game_over_images:[&'static [u8;320000]; 2]
    ) -> Graphics {
        Graphics {
            display_layer: display_layer,
            width: width,
            height: height,
            numbers: numbers,
            game_over_images: game_over_images
        }
    }

    /* /// check if point is outside
    fn isPointOutside(&self, point: [u16; 2], pointsize: u16) -> bool {
        return (self.width > point[0] + pointsize || point[0] - pointsize < 0)
            && (self.height > point[1] + pointsize || point[1] > 0);
    } */
    fn draw_circle_in_field(
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
                        .0
                        .print_point_color_at(x, y, Color::from_hex(color));
                }
            }
        }
    }

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
    pub fn clear_layer(&mut self, layer: u8) {
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
    pub fn draw_score(&mut self, player_scores: Vec<u16>) {
        // Draws two number and : 2:1

        // First number
        self.draw_number([190, 15], player_scores[0]as u8);
        // Second number
        self.draw_number([265, 15], player_scores[1] as u8);

        // Double dot
    }


    pub fn draw_game_over(&mut self, player: u8) {

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
        self.draw_circle_in_field(
            self::DOUBLE_DOT_COLOR,
            [240, 25],
            2,
            false,
            self::DOUBLE_DOT_COLOR,
        );
        self.draw_circle_in_field(
            self::DOUBLE_DOT_COLOR,
            [240, 35],
            2,
            false,
            self::DOUBLE_DOT_COLOR,
        );
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
            field_size[0] / 2 - border_width / 4,
            0,
            field_size[0] / 2 + border_width / 4,
            field_size[1],
            color,
        );
    }

    pub fn draw_circles_implicit(&mut self, circles: &Vec<((u16, u16), u16, u32)>) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut in_circle = false;
                for circle in circles {
                    if self.in_circle(x, y, [(circle.0).0, (circle.0).1], circle.1) {
                        (self.display_layer.1).print_point_color_at(
                            x as usize,
                            y as usize,
                            Color::from_hex(circle.2),
                        );
                        in_circle = true;
                    }
                }
                if !in_circle {
                    (self.display_layer.1).print_point_color_at(
                        x as usize,
                        y as usize,
                        Color::from_argb8888(0),
                    );
                }
            }
        }
    }

    fn in_circle(&self, x: u16, y: u16, pos: [u16; 2], radius: u16) -> bool {
        (i32::from(x) - i32::from(pos[0])) * (i32::from(x) - i32::from(pos[0]))
            + (i32::from(y) - i32::from(pos[1])) * (i32::from(y) - i32::from(pos[1]))
            <= i32::from(radius) * i32::from(radius)
    }


    /// draw numbers on screen at positi
    pub fn draw_number(
        &mut self,
        // upper left position of number rectangle
        position: [u16; 2],
        score: u8, // Dimension of number rectangle
    ) {
        let mut x_pos: u16;
        let mut y_pos: u16;
        let mut color_score:lcd::Color;
        for i in 0..1000 {
            // unpack 1darray to 2darray
            x_pos = i % self::NUMBER_WIDTH;
            y_pos = i / self::NUMBER_HIGHT;

            color_score=lcd::Color::rgba(
            self.numbers[score as usize][(4*i) as usize],
            self.numbers[score as usize][((4*i)+1) as usize],
            self.numbers[score as usize][((4*i)+2) as usize],
            self.numbers[score as usize][((4*i)+3) as usize]);


            self.display_layer.0.print_point_color_at(
                x_pos as usize + position[0] as usize,
                y_pos as usize + position[1] as usize,
                color_score,
            );
        }
    }

    /// draw end_game
    pub fn draw_endgame(
        &mut self,
        // array with the information of number
        position: [u32; 2],
        winning_player: u8
        // Dimension of number rectangle
    ) {
        let mut x_pos: u32;
        let mut y_pos: u32;
        let mut color_score:lcd::Color=lcd::Color::rgb(
            0, 0, 0
            );
        for i in 0..8000 {
            x_pos = i % self::END_WIDTH as u32;
            y_pos = i / self::END_HIGHT as u32;

            color_score=lcd::Color::rgba(
            self.numbers[winning_player as usize][(4*i) as usize],
            self.numbers[winning_player as usize][((4*i)+1) as usize],
            self.numbers[winning_player as usize][((4*i)+2) as usize],
            self.numbers[winning_player as usize][((4*i)+3) as usize]
            );

            // color_score.red=self.numbers[winning_player as usize][(4*i) as usize];
            // color_score.green=self.numbers[winning_player as usize][(4*i)+1 as usize];
            // color_score.blue=self.numbers[winning_player as usize][(4*i)+2 as usize];
            // color_score.alpha=self.numbers[winning_player as usize][(4*i)+3 as usize];


            self.display_layer.0.print_point_color_at(
                x_pos as usize + position[0] as usize,
                y_pos as usize + position[1] as usize,
                color_score,
            );
        }
    }

}

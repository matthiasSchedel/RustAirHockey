//! Graphics controller.
use crate::lcd::{self, FramebufferArgb8888};

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

    /// draw a circle around pos x,y with radius - and
    pub fn draw_circle(
        &self,
        color: u32,
        pos: [u16; 2],
        radius: u16,
        draw_stroke: bool,
        stroke_color: u32,
    ) {
        for y in pos[1] - radius..=pos[1] + radius {
            for x in pos[0] - radius..=pos[0] + radius {
                if x * x + pos[0] * pos[0] - 2 * x * pos[0] + y * y + pos[1] * pos[1]
                    - 2 * y * pos[1]
                    <= radius * radius
                {
                    //layer.print_point_color_at(x as usize , y as usize , color);
                }
            }
        }
    }
    ///  clear a circle
    pub fn clear_circle(&self, color: u16, pos: [u16; 2], radius: f32) {}

    ///  clear the field
    pub fn clear_field(&self, color: u16) {}

    /// draw a field
    pub fn draw_field(&self, field_color: u16, border_color: u16, goal_size: u16) {}

    /// draw a score
    pub fn draw_score(&self, player1_score: u8, player2_score: u8) {}

    /// init
    pub fn init(&self) {}

    /// draw a rectangle
    pub fn draw_rectangle(
        layer: &mut lcd::Layer<FramebufferArgb8888>,
        x_start: u16,
        y_start: u16,
        x_end: u16,
        y_end: u16,
        color: lcd::Color,
    ) {
        for x in x_start..x_end {
            for y in y_start..y_end {
                layer.print_point_color_at(x as usize, y as usize, color);
            }
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

//     // upper rectangle
//     draw_rectangle(layer, 0  , HEIGHT-width  , WIDTH  , HEIGHT  , color);

//     // left side
//     draw_rectangle(layer, 0  , 0  , width  , (HEIGHT-goal_size)/2  , color);
//     draw_rectangle(layer, 0  , (HEIGHT+goal_size)/2  , width  ,  HEIGHT  , color);

//     // draw right side
//     draw_rectangle(layer, WIDTH-width  , 0  , WIDTH  , (HEIGHT-goal_size)/2  , color);
//     draw_rectangle(layer, WIDTH-width  , (HEIGHT+goal_size)/2   , WIDTH  ,  HEIGHT  , color);

// }

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

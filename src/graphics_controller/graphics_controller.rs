//! Graphics controller.
const STROKE_COLOR: u32 = 0xffff00;
const USE_STROKE: bool = true;
const PLAYER_SIZE: u16 = 10;
const PUCK_SIZE: u16 = 6;
const BACKGROUND_COLOR: u32 = 0xfff000;

use crate::lcd::FramebufferAl88;
// GraphicsController struct
pub struct GraphicsController {
    display: i32,
    // display width
    width: u16,
    //display height
    height: u16,
}
impl GraphicsController {
    // game constructor
    pub fn new(width: u16, height: u16) -> GraphicsController {
        GraphicsController {
            display: 2,
            width: width,
            height: height,
        }
    }
    // is touched method
    pub fn is_touched(&self, p_id: usize) -> bool {
        return false;
    }

    //draw a circle around pos x,y with radius - and
    pub fn draw_circle(
        &self,
        color: u32,
        pos: [u16;2],
        radius: u16,
        draw_stroke: bool,
        stroke_color: u32,
    ) {
        for y in pos[1] - radius..=pos[1] + radius {
            for x in pos[0] - radius..=pos[0] + radius {
                if x * x + pos[0] * pos[0] - 2 * x * pos[0] + y * y
                    + pos[1] * pos[1] - 2 * y * pos[1]
                    <= radius * radius
                {
                //layer.print_point_color_at(x as usize , y as usize , color);
            }
        }

    }

    pub fn clear_circle(&self, color: u16, pos[0]: u16, pos[1]: u16, radius: f32) {}

    pub fn clear_field(&self, color: u16) {}

    pub fn draw_field(&self, field_color: u16, border_color: u16, goal_size: u16) {}

    pub fn draw_score(&self, player1_score: u8, player2_score: u8) {}

    pub fn init(&self) {}
}

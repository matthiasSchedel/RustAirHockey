//! graphics handler module
use super::field;
use super::{ball, ball::Ball};
use super::{player, player::Player};
use crate::graphics::graphics::Graphics;
use alloc::vec::Vec;

/// no of layer where to draw
const DRAW_LAYER_NO: u8 = 0;

/// graphics handler struct
pub struct GraphicsHandler {
    graphics: Graphics,
    screen_size: [u16; 2],
}

impl GraphicsHandler {
    /// handler constructor
    pub fn new(graphics: Graphics) -> GraphicsHandler {
        GraphicsHandler {
            graphics: graphics,
            screen_size: [480, 272],
        }
    }

    /// draw the ball
    pub fn draw_ball(&mut self, color: u32, pos: [u16; 2], radius: u16) {
        self.graphics.draw_circle(color, pos, radius, false, color);
    }

    pub fn draw_player(&mut self, color: u32, pos: [u16; 2], radius: u16) {
        self.graphics.draw_circle(color, pos, radius, false, color);
    }

    pub fn draw_score(&mut self, player_scores: Vec<u16>) {
        self.graphics.draw_score(player_scores)
    }

    pub fn draw_playing_field(&mut self) {
        self.graphics.draw_field(
            field::BACKGROUND_COLOR,
            [field::WIDTH_MAX, field::HEIGHT_MAX],
            field::BORDER_WIDTH,
            field::GOAL_SIZE,
        );
    }

    pub fn draw_game_content(&mut self, drawables: &Vec<((u16, u16), u16, u32)>) {
        self.graphics.draw_circles_implicit(drawables);
    }

    pub fn draw_game_over(&mut self, winnning_player: u8) {
        self.graphics.draw_endgame([40, 36], winnning_player);
    }

    pub fn clear_dynamic_content(&mut self) {
        self.graphics.clear_layer(0);
    }
}

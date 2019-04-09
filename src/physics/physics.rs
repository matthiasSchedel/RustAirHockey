//! Graphics controller.
use crate::alloc;
use alloc::vec::Vec;

/// friction used in simulation
const FRICTION: u32 = 0xffff00;
/// is friction used in simulation?
const USE_FRICTION: bool = true;

/// Physics struct
pub struct Physics {
    /// display
    display: i32,
    // display width
    width: u16,
    //display height
    height: u16,
    ball_pos: Vec<u16>,
}

/// collisions object struct
pub struct CollisionObject {
    has_collided: bool,
    collision_pos: Vec<u16>,
}

impl CollisionObject {
    /// game constructor
    pub fn new(has_collided: bool, pos_x: u16, pos_y: u16) -> CollisionObject {
        CollisionObject {
            has_collided: has_collided,
            collision_pos: vec![pos_x, pos_y],
        }
    }
}

impl Physics {
    /// physics constructor
    pub fn new(width: u16, height: u16) -> Physics {
        Physics {
            display: 2,
            width: width,
            height: height,
            ball_pos: vec![0, 0],
        }
    }

    /// set the ball position
    pub fn set_ball_pos(&self, pos_x: u16, pos_y: u16) {
        // self.ball_pos = vec![pos_x, pos_y];
    }

    /// update the ball position
    pub fn update_ball_position(&self, old_x: u16, old_y: u16, radius: u16, coll: CollisionObject) {
        let border_collisions: CollisionObject =
            self.calculate_border_collision_point(old_x, old_y, radius);
        if border_collisions.has_collided {
            self.update_pos_from_coll_point(border_collisions);
        } else {
            self.update_ball_pos_without_coll();
        }
    }
    /// update the ball position from coll point
    fn update_pos_from_coll_point(&self, coll: CollisionObject) {}

    /// update the ball position without coll point
    fn update_ball_pos_without_coll(&self) {}

    /// calc border coll point
    fn calculate_border_collision_point(
        &self,
        old_x: u16,
        old_y: u16,
        radius: u16,
    ) -> CollisionObject {
        return CollisionObject::new(false, old_x, old_y);
    }

    /// input some circle object
    pub fn calculate_circle_coll_with_ball(&self) {}
}

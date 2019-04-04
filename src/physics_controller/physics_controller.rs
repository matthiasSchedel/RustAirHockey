//! Graphics controller.
use crate::alloc;
use alloc::vec::Vec;

const FRICTION: u32 = 0xffff00;
const USE_FRICTION: bool = true;

// PhysicsController struct
pub struct PhysicsController {
    display: i32,
    // display width
    width: u16,
    //display height
    height: u16,
    ball_pos: Vec<u16>,
}

pub struct CollisionObject {
    has_collided: bool,
    collision_pos: Vec<u16>,
}

impl CollisionObject {
    // game constructor
    pub fn new(has_collided: bool, pos_x: u16, pos_y: u16) -> CollisionObject {
        CollisionObject {
            has_collided: has_collided,
            collision_pos: vec![pos_x, pos_y],
        }
    }
}

impl PhysicsController {
    // game constructor
    pub fn new(width: u16, height: u16) -> PhysicsController {
        PhysicsController {
            display: 2,
            width: width,
            height: height,
            ball_pos: vec![0, 0],
        }
    }

    pub fn set_ball_pos(&self, pos_x: u16, pos_y: u16) {
        // self.ball_pos = vec![pos_x, pos_y];
    }

    pub fn update_ball_position(&self, old_x: u16, old_y: u16, radius: u16, coll: CollisionObject) {
        let border_collisions: CollisionObject =
            self.calculate_border_collision_point(old_x, old_y, radius);
        if border_collisions.has_collided {
            self.update_pos_from_coll_point(border_collisions);
        } else {
            self.update_ball_pos_without_coll();
        }
    }
    fn update_pos_from_coll_point(&self, coll: CollisionObject) {}

    fn update_ball_pos_without_coll(&self) {}

    fn calculate_border_collision_point(
        &self,
        old_x: u16,
        old_y: u16,
        radius: u16,
    ) -> CollisionObject {
        return CollisionObject::new(false, old_x, old_y);
    }

    //input some circle object
    pub fn calculate_circle_coll_with_ball(&self) {}
}

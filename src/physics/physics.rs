//! Graphics controller.
use crate::alloc;
use m::Float;
use alloc::vec::Vec;

const FRICTION: u32 = 0xff_ff_00;
const USE_FRICTION: bool = true;

// Physics struct
pub struct Physics {
    display: i32,
    // display width
    width: u16,
    //display height
    height: u16,
    ball_pos: Vec<u16>,
    ball_speed: Vec<f32>,
    ball_radius: u16,
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

impl Physics {
    // game constructor
    pub fn new(width: u16, height: u16, radius: u16) -> Physics {
        Physics {
            display: 2,
            width: width,
            height: height,
            ball_pos: vec![0, 0],
            ball_speed: vec![0., 0.],
            ball_radius: radius,
        }
    }

    pub fn set_ball_pos(&self, pos_x: u16, pos_y: u16) {
        self.ball_pos = vec![pos_x, pos_y];
    }

    pub fn update_ball_position(&mut self, old_x: u16, old_y: u16, radius: u16, coll: CollisionObject) {
        let border_collisions: CollisionObject =
            self.calculate_border_collision_point(old_x, old_y, radius);

        // this one needs a fix, as it might need other parameters.
        let player_collision : CollisionObject =
            self.calculate_ball_collision_point(old_x, old_y, radius);

        if border_collisions.has_collided {
            self.update_pos_from_coll_point(border_collisions);
        } else if player_collision.has_collided {
            self.update_pos_from_coll_point(player_collision);
        } else {
            self.update_ball_pos_without_coll();
        }
    }

    fn update_pos_from_coll_point(&self, coll: CollisionObject) {
        // border-collision
        if coll.collision_pos[0] == 0 || coll.collision_pos[0] == self.width {
            self.ball_speed[0] *= -1.0;
        }
        if coll.collision_pos[1] == 0 || coll.collision_pos[1] == self.height {
            self.ball_speed[1] *= -1.0;
        }
        // TODO: ball-collision
        // migrate from examples/display.rs - we might need speed information somewhere


        // collision is handled - update position
        self.ball_pos[0] += self.ball_speed[0] as u16;
        self.ball_pos[1] += self.ball_speed[1] as u16;

    }

    fn update_ball_pos_without_coll(&mut self) {
        self.ball_pos[0] += self.ball_speed[0] as u16;
        self.ball_pos[1] += self.ball_speed[1] as u16;
    }

    /// checks if and where a ball collides with the border and returns a corresponding collision object
    fn calculate_border_collision_point(
        &self,
        old_x: u16,
        old_y: u16,
        radius: u16,
    ) -> CollisionObject {
        let coll_x;
        let coll_y;
        let mut collision: bool = false;

        //x-Richtung: Fallen wir links raus? Rechts?
        if i32::from(old_x) + self.ball_speed[0] as i32 + i32::from(radius) <= 0 {
            collision = true;
            coll_x = 0;
        } else if old_x + self.ball_speed[0] as u16 + radius >= self.width {
            collision = true;
            coll_x = self.width;
        }  else {
            coll_x = old_x;
        }

        //y-Richtung: Fallen wir oben oder unten raus?
        if i32::from(old_y) + self.ball_speed[1] as i32+ i32::from(radius) <= 0 {
            collision = true;
            coll_y = 0;
        } else if old_y + self.ball_speed[1] as u16 + radius >= self.height {
            collision = true;
            coll_y = self.height;
        } else {
            coll_y = old_y;
        }

        CollisionObject::new(collision, coll_x, coll_y)
    }

    fn calculate_point_distance(&self, position1: Vec<u16>, position2: Vec<u16>) -> f32 {
        f32::from((position1[0] - position2[0])*(position1[0]-position2[0]) + (position1[1]-position2[1])*(position1[1]-position2[1])).sqrt()
    }

    /// checks if and where a ball collides with a player and returns a corresponding collision object
    fn calculate_ball_collision_point(
        &self,
        old_x: u16,
        old_y: u16,
        radius: u16,
    ) -> CollisionObject {
        let player_pos = vec![old_x, old_y];
        let collision = self.calculate_point_distance(player_pos, self.ball_pos) <= (radius + self.ball_radius).into();

        CollisionObject::new(collision, old_x, old_y)
    }
    //input some circle object
    pub fn calculate_circle_coll_with_ball(&self) {}

}

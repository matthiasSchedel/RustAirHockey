//! Graphics controller.
extern crate m;
use crate::alloc;
use m::Float;
use alloc::vec::Vec;

const FRICTION: u32 = 0xff_ff_00;
const USE_FRICTION: bool = true;

/// Physics struct
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
    collision_speed: Vec<f32>,
}

impl CollisionObject {
    /// game constructor
    pub fn new(has_collided: bool, pos_x: u16, pos_y: u16, speed_x: f32, speed_y: f32) -> CollisionObject {
        CollisionObject {
            has_collided: has_collided,
            collision_pos: vec![pos_x, pos_y],
            collision_speed: vec![speed_x, speed_y],

        }
    }
}

impl Physics {
    /// game constructor
    pub fn new(width: u16, height: u16, radius: u16) -> Physics {
        Physics {
            display: 2,
            width,
            height,
            ball_pos: vec![0, 0],
            ball_speed: vec![0., 0.],
            ball_radius: radius,
        }
    }

    /// beams the ball to a given position. Instant speed.
    pub fn set_ball_pos(&mut self, pos_x: u16, pos_y: u16) {
        self.ball_pos = vec![pos_x, pos_y];
    }

    /// updates the position (and speed) of the ball.
    /// (1) check for collision wit border or with another object
    /// (2) adjust speed depending on (1)
    /// (3) update position
    pub fn update_ball_position(&mut self, old_x: u16, old_y: u16, radius: u16, speed_x: f32, speed_y: f32, coll: CollisionObject) {
        let border_collisions: CollisionObject =
            self.calculate_border_collision_point(old_x, old_y, radius);

        // this one needs a fix, as it might need other parameters.
        let player_collision : CollisionObject =
            self.calculate_ball_collision_point(old_x, old_y, radius, speed_x, speed_y);

        if border_collisions.has_collided {
            self.update_pos_from_coll_point(border_collisions);
        } else if player_collision.has_collided {
            self.update_pos_from_coll_point(player_collision);
        } else {
            self.update_ball_pos_without_coll();
        }
    }

    fn update_pos_from_coll_point(&mut self, coll: CollisionObject) {
        // border-collision
        if coll.collision_pos[0] == 0 || coll.collision_pos[0] == self.width {
            self.ball_speed[0] *= -1.0;
        }
        if coll.collision_pos[1] == 0 || coll.collision_pos[1] == self.height {
            self.ball_speed[1] *= -1.0;
        }

        // player-collision
        if coll.collision_speed[0] != 0. {
            self.ball_speed[0] = coll.collision_speed[0];
        }

        if coll.collision_speed[1] != 0. {
            self.ball_speed[1] = coll.collision_speed[1];
        }

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

        CollisionObject::new(collision, coll_x, coll_y, 0., 0.)
    }

    fn calculate_point_distance(&self, position1: Vec<u16>) -> f32 {
        f32::from((position1[0] - self.ball_pos[0])*(position1[0]-self.ball_pos[0]) + (position1[1]-self.ball_pos[1])*(position1[1]-self.ball_pos[1])).sqrt()
    }

    /// checks if and where a ball collides with a player and returns a corresponding collision object
    fn calculate_ball_collision_point(
        &mut self,
        old_x: u16,
        old_y: u16,
        radius: u16,
        speed_x: f32,
        speed_y: f32,
    ) -> CollisionObject {
        let player_pos = vec![old_x, old_y];
        let collision = self.calculate_point_distance(player_pos) <= (radius + self.ball_radius).into();

        //here be physics
        let mut norm_x: f32 = f32::from(self.ball_pos[0]) - f32::from(old_x);
        let mut norm_y: f32 = f32::from(self.ball_pos[1]) - f32::from(old_y);

        let dist: f32 = norm_x*norm_x + norm_y*norm_y.sqrt();
        
        norm_x /= dist;
        norm_y /= dist;

        let tan_x = norm_y;
        let tan_y = -norm_x;

        let overlap = self.ball_radius + radius - dist as u16;

        // beides etwas auseinander schieben, um nicht sofort wieder zu kollidieren
        self.ball_pos[0] -= norm_x as u16 * overlap as u16;
        self.ball_pos[1] -= norm_y as u16 * overlap as u16;
        // player könnte auch geschoben werden, aber da da im Zweifel ein Finger drauf ist, ergibt das wenig Sinn hier

        // wir drehen das Bezugssystem passend zur Kollision.
        // erstmal für den Ball
        let velocity_norm = norm_x * self.ball_speed[0] + norm_y * self.ball_speed[1];
        let velocity_tan = tan_x * self.ball_speed[0] + tan_y * self.ball_speed[1];

        let share_norm_x = norm_x * velocity_norm;
        let share_norm_y = norm_y * velocity_norm;

        let share_tan_x = tan_x * velocity_tan;
        let share_tan_y = tan_y * velocity_tan;

        // jetzt für den Player
        let velocity_norm_player = norm_x * speed_x + norm_y * speed_y;
        let velocity_tan_player = tan_x as i16 * speed_x as i16 + tan_y as i16 * speed_y as i16;

        let share_norm_x_player = norm_x * velocity_norm_player;
        let share_norm_y_player = norm_y * velocity_norm_player;

        let share_tan_x_player = tan_x as i16 * velocity_tan_player;
        let share_tan_y_player = tan_y * velocity_norm_player;

        // jetzt wird geschummelt. Nur der Ball bekommt den Bounce, that's not how to physics
        let pulse_x = share_norm_x + share_norm_x_player;
        let pulse_y = share_norm_y + share_norm_y_player;

        let coll_speed_x = pulse_x - share_norm_x + share_tan_x;
        let coll_speed_y = pulse_y - share_norm_y + share_tan_y;

        CollisionObject::new(collision, old_x, old_y, coll_speed_x, coll_speed_y)
    }
    ///input some circle object
    pub fn calculate_circle_coll_with_ball(&self) {}

}

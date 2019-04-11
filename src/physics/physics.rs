//! Graphics controller.
extern crate libm;
use libm::F64Ext;
use crate::airhockey::helper;

const FRICTION: f64 = 1.;
/// friction used in simulation
/// is friction used in simulation?
const USE_FRICTION: bool = true;

/// Physics struct
pub struct Physics {
    // display width
    width: u16,
    //display height
    height: u16,
    ball_pos: (u16, u16),
    ball_speed: (f64, f64),
    ball_radius: u16,
}

/// contains all relevat information an a collision
/// collisions object struct
pub struct CollisionObject {
    has_collided: bool,
    collision_pos: (u16, u16),
    collision_speed: (f64, f64),
}

impl CollisionObject {
    /// game constructor
    pub fn new(
        has_collided: bool,
        pos_x: u16,
        pos_y: u16,
        speed_x: f64,
        speed_y: f64,
    ) -> CollisionObject {
        CollisionObject {
            has_collided,
            collision_pos: (pos_x, pos_y),
            collision_speed: (speed_x, speed_y),
        }
    }
}

impl Physics {
    /// physics constructor
    pub fn new(width: u16, height: u16, ball_radius: u16) -> Physics {
        Physics {
            width,
            height,
            ball_pos: (200, 200),
            ball_speed: (0., 0.),
            ball_radius,
        }
    }

    /// beams the ball to a given position. Instant speed.
    pub fn set_ball_pos(&mut self, &pos_x: &u16, &pos_y: &u16) {
        self.ball_pos.0 = pos_x;
        self.ball_pos.1 = pos_y;
    }

    /// sets ball speed
    pub fn set_ball_speed(&mut self, &speed_x: &f64, &speed_y: &f64) {
        self.ball_speed.0 = speed_x;
        self.ball_speed.1 = speed_y;
    }

    /// updates the position (and speed) of the ball.
    /// (1) check for collision wit border or with another object
    /// (2) adjust speed depending on (1)
    /// (3) update position
    pub fn update_ball_position(
        &mut self,
        player_x: u16,
        player_y: u16,
        player_radius: u16,
        speed_x: f64,
        speed_y: f64,
    ) -> (u16, u16) {
        if self.ball_speed.0 > 2. * f64::from(player_radius) {
            self.ball_speed.0 = 1.95 * f64::from(player_radius);
        }

        if self.ball_speed.1 > 2. * f64::from(player_radius) {
            self.ball_speed.1 = 1.95 * f64::from(player_radius) ;
        }
        let border_collisions: CollisionObject =
            self.calculate_border_collision_point();

        // this one needs a fix, as it might need other parameters.
        let player_collision: CollisionObject = self.calculate_ball_collision_point(
            player_x,
            player_y,
            player_radius,
            speed_x,
            speed_y,
        );

        if border_collisions.has_collided {
            self.update_pos_from_coll_point(border_collisions)
        } else if player_collision.has_collided {
            self.update_pos_from_coll_point(player_collision)
        } else {
            self.update_ball_pos_without_coll(player_x, player_y, player_radius)
        }
    }

    fn update_pos_from_coll_point(&mut self, coll: CollisionObject) -> (u16, u16) {
        // border-collision
        if coll.collision_pos.0 == 0 || coll.collision_pos.0 == self.width {
            self.ball_speed.0 *= -1.0;
        }
        if coll.collision_pos.1 == 0 || coll.collision_pos.1 == self.height {
            self.ball_speed.1 *= -1.0;
        }

        // player-collision
        if coll.collision_speed.0 != 0. {
            self.ball_speed.0 = coll.collision_speed.0;
        }

        if coll.collision_speed.1 != 0. {
            self.ball_speed.1 = -coll.collision_speed.1;
        }

        // collision is handled - update position
        self.ball_pos.0 = (f64::from(self.ball_pos.0) + self.ball_speed.0) as u16;
        self.ball_pos.1 = (f64::from(self.ball_pos.1) + self.ball_speed.1) as u16;

        (self.ball_pos.0, self.ball_pos.1)
    }

    fn update_ball_pos_without_coll(&mut self, player_x: u16, player_y: u16, player_radius: u16) -> (u16, u16) {
        
        let bias: (u16, u16) = self.proactive_ball_player_collision_check(player_x, player_y, player_radius);
        
        //set new position
        self.ball_pos.0 += bias.0;
        self.ball_pos.1 += bias.1;

        //apply friction
        if USE_FRICTION {
            self.ball_speed.0 *= FRICTION;
            self.ball_speed.1 *= FRICTION;
        }

        (self.ball_pos.0, self.ball_pos.1)
    }

    /// checks if and where a ball collides with the border and returns a corresponding collision object
    fn calculate_border_collision_point(&self) -> CollisionObject {
        let coll_x;
        let coll_y;
        let mut collision: bool = false;

        //x-Richtung: Fallen wir links raus? Rechts?
        if i32::from(self.ball_pos.0) + self.ball_speed.0 as i32 - i32::from(self.ball_radius) <= 10 {
            collision = true;
            coll_x = 0;
        } else if i32::from(self.ball_pos.0) + self.ball_speed.0 as i32 + i32::from(self.ball_radius) >= i32::from(self.width) -10{
            collision = true;
            coll_x = self.width;
        } else {
            coll_x = self.ball_pos.0;
        }

        //y-Richtung: Fallen wir oben oder unten raus?
        if i32::from(self.ball_pos.1) + self.ball_speed.1 as i32 - i32::from(self.ball_radius) <= 10 {
            collision = true;
            coll_y = 0;
        } else if i32::from(self.ball_pos.1) + self.ball_speed.1 as i32 + i32::from(self.ball_radius) >= i32::from(self.height) -10 {
            collision = true;
            coll_y = self.height;
        } else {
            coll_y = self.ball_pos.1;
        }

        CollisionObject::new(collision, coll_x, coll_y, 0., 0.)
    }

    //Pythagoras
    fn calculate_point_distance(&self, position1: (u16, u16)) -> f64 {
        let x:f64 = f64::from(
            u32::from(helper::unsigned_subtraction(position1.0, self.ball_pos.0)) * u32::from(helper::unsigned_subtraction(position1.0, self.ball_pos.0))
                + u32::from(helper::unsigned_subtraction(position1.1, self.ball_pos.1)) * u32::from(helper::unsigned_subtraction(position1.1, self.ball_pos.1))
        );
        x.sqrt()
    }

    /// checks if and where a ball collides with a player and returns a corresponding collision object
    fn calculate_ball_collision_point (
        &mut self,
        player_x: u16,
        player_y: u16,
        player_radius: u16,
        speed_x: f64,
        speed_y: f64,
    ) -> CollisionObject {
        let player_pos = (player_x, player_y);
        let collision =
            self.calculate_point_distance(player_pos) <= (player_radius + self.ball_radius).into();

        if !collision {
            return CollisionObject::new(false, player_x, player_y, speed_x, speed_y);           
        }

        //here be physics
        let mut norm_x: f64 = f64::from(self.ball_pos.0) - f64::from(player_x);
        let mut norm_y: f64 = f64::from(self.ball_pos.1) - f64::from(player_y);

        let mut dist: f64 = (norm_x * norm_x + norm_y * norm_y).sqrt();

        // check there is at least some distance
        if dist < 0.000_000_119_209_29 /*that is EPSILON - but we don't have std::f32, so I inlined it.*/ {
            // move ball out of player. update norm_x and dist
            self.ball_pos.0 += player_radius;
            norm_x += f64::from(player_radius);
            dist = (norm_x * norm_x + norm_y * norm_y).sqrt();
        }

        norm_x /= dist;
        norm_y /= dist;

        let tan_x = norm_y;
        let tan_y = -norm_x;

        let overlap = (2.01*(f64::from(self.ball_radius) + f64::from(player_radius) - dist)) as i32;

        // falls es overlap gibt, beides etwas auseinander schieben, um nicht sofort wieder zu kollidieren
        if overlap > 0 {
            if self.ball_speed.0 > 0. {
                self.ball_pos.0 -= norm_x as u16 * overlap as u16;
            } else {
                self.ball_pos.0 += norm_x as u16 * overlap as u16;
            }
            
            if self.ball_speed.1 > 0. {
                self.ball_pos.1 -= norm_y as u16 * overlap as u16;
            } else {
                self.ball_pos.1 += norm_y as u16 * overlap as u16;
            }
        }
        


        // player könnte auch geschoben werden, aber da da im Zweifel ein Finger drauf ist, ergibt das wenig Sinn hier

        // wir drehen das Bezugssystem passend zur Kollision.
        // erstmal für den Ball
        let velocity_norm = norm_x * self.ball_speed.0 + norm_y * self.ball_speed.1;
        let velocity_tan = tan_x * self.ball_speed.0 + tan_y * self.ball_speed.1;

        let share_norm_x = norm_x * velocity_norm;
        let share_norm_y = norm_y * velocity_norm;

        let share_tan_x = tan_x * velocity_tan;
        let share_tan_y = tan_y * velocity_tan;

        // jetzt für den Player
        let velocity_norm_player = norm_x * speed_x + norm_y * speed_y;
        let velocity_tan_player = tan_x as i16 * speed_x as i16 + tan_y as i16 * speed_y as i16;

        let share_norm_x_player = norm_x * velocity_norm_player;
        let share_norm_y_player = norm_y * velocity_norm_player;

        // _, weil wir sie aktuell nicht verwenden
        let _share_tan_x_player = tan_x as i16 * velocity_tan_player;
        let _share_tan_y_player = tan_y * velocity_norm_player;

        // jetzt wird geschummelt. Nur der Ball bekommt den Bounce, that's not how to physics
        let pulse_x = share_norm_x + share_norm_x_player;
        let pulse_y = share_norm_y + share_norm_y_player;

        let coll_speed_x = if pulse_x - share_norm_x + share_tan_x > 60. {60.} else {(pulse_x - share_norm_x + share_tan_x)};
        let coll_speed_y = if pulse_y - share_norm_y + share_tan_y > 60. {60.} else {(pulse_y - share_norm_y + share_tan_y)};

        CollisionObject::new(collision, player_x, player_y, coll_speed_x, coll_speed_y)
    }
    ///input some circle object
    pub fn calculate_circle_coll_with_ball(&self) {}

    ///get ball speed
    pub fn get_ball_speed(&self) -> (f64, f64) {
        (self.ball_speed.0, self.ball_speed.1)
    }

    /// check for possible collision before setting new position
    /// returns a for the normalized speed vector. i.e.: collision happens a * norm(speed) + position
    pub fn proactive_ball_player_collision_check(&self, player_x: u16, player_y: u16, player_radius: u16) -> (u16, u16) {
        
        // für alle Linien gleich
        let r = (self.ball_speed.0, self.ball_speed.1);
        let r_length = ((self.ball_speed.0*self.ball_speed.0)+(self.ball_speed.1*self.ball_speed.1)).sqrt();
        let r_norm = (r.0/r_length, r.1/r_length);


        // Fußpunkte zur Kollisionserkennung
        // +radius
        let f1 = (0.,0.);

        // durch den Mittelpunkt
        let f2 = (0.,0.);
        
        // -radius 
        let f3 = (0.,0.);

        for a in 0..=r_length as u32 {
            let p1 = (f1.0+f64::from(a) * r_norm.0, f1.1 + f64::from(a) * r_norm.1);
            let p2 = (f2.0+f64::from(a) * r_norm.0, f2.1 + f64::from(a) * r_norm.1);
            let p3 = (f3.0+f64::from(a) * r_norm.0, f3.1 + f64::from(a) * r_norm.1);

            if self.in_circle(p1.0, p1.1, (player_x, player_y), player_radius) 
                || self.in_circle(p2.0, p2.1, (player_x, player_y), player_radius) 
                || self.in_circle(p3.0, p3.1, (player_x, player_y), player_radius) {
                return ((f64::from(a) * r_norm.0) as u16, (f64::from(a) * r_norm.1) as u16);
            }
        }

        (r.0 as u16 ,r.1 as u16)
    }

    fn in_circle(&self, x: f64, y: f64, center: (u16, u16), radius: u16) -> bool {
        (x  - f64::from(center.0)) * (x - f64::from(center.0)) + (y - f64::from(center.1))*(y- f64::from(center.1))  
            <= f64::from(radius)*f64::from(radius)
    }


}

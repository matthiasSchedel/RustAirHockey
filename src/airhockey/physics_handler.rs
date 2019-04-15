//! physics handler module
use crate::airhockey::score::Score;
use crate::physics::physics::Physics;

/// physics handler struct
pub struct PhysicsHandler {
    pub physics: Physics,
    screen_size: [u16; 2],
}

impl PhysicsHandler {
    /// physics handler constructor
    pub fn new(physics: Physics) -> PhysicsHandler {
        PhysicsHandler {
            physics,
            screen_size: [480, 272],
        }
    }

    /// sets the ball in the physics object
    pub fn update_ball_speed(&mut self, new_speed: &(f64, f64)) {
        self.physics.set_ball_speed(&new_speed.0, &new_speed.1);
    }

    /// checks for collisions and calculates and returns new ball position
    ///  accordingly
    pub fn update_ball_position(
        &mut self,
        player_position: (u16, u16),
        player_radius: u16,
        player_speed: (f64, f64),
        score: &mut Score,
    ) -> (u16, u16) {
        self.physics.update_ball_position(
            player_position.0,
            player_position.1,
            player_radius,
            player_speed.0,
            player_speed.1,
            score
        )
    }

    /// returns the current ball speed from the physics objfect
    pub fn get_ball_speed(&self) -> (f64, f64) {
        self.physics.get_ball_speed()
    }
}

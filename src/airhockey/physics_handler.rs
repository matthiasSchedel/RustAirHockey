//! physics handler module
use crate::physics::physics::Physics;

/// physics handler struct
pub struct PhysicsHandler {
  physics: Physics,
  screen_size: [u16; 2],
}

impl PhysicsHandler {
  /// physics handler constructor
  pub fn new(physics: Physics) -> PhysicsHandler {
    PhysicsHandler {
      physics: physics,
      screen_size: [480, 272],
    }
  }
  // Ball constructor

  /// (true,_) if collision, (,_) and new speed after collection
  pub fn calculate_border_coll(pos: [u16; 2]) -> (bool, [u16; 2]) {
    return { (false, [0, 0]) };
  }
}

use std::time::Duration;

use bevy::prelude::*;

// Define a struct to keep some information about our entity.
// Here it's an arbitrary movement speed, the spawn location, and a maximum distance from it.
#[derive(Component)]
pub struct Movable {
    pub direction: Vec3,
    pub forward_speed: f32,
}

// Implement a utility function for easier Movable struct creation.
impl Movable {
    pub fn new() -> Self {
        Movable {
            direction: Vec3::new(1., 0., 0.),
            forward_speed: 0.,
        }
    }
}

#[derive(Component)]
pub struct ProximityActivated {
    pub is_activated: bool,
    pub detection_radius: f32,
    pub elapsed_activated: Duration,
}

impl ProximityActivated {
    pub fn new() -> Self {
        ProximityActivated {
            is_activated: false,
            detection_radius: 0.8,
            elapsed_activated: Duration::ZERO,
        }
    }
}

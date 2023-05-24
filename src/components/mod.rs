use std::time::Duration;

use bevy::prelude::*;

// Define a struct to keep some information about our entity.
// Here it's an arbitrary movement speed, the spawn location, and a maximum distance from it.
#[derive(Component)]
pub struct Movable {
    pub forward_speed: f32,
}

// Implement a utility function for easier Movable struct creation.
impl Movable {
    pub fn new() -> Self {
        Movable { forward_speed: 0. }
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
            detection_radius: 2.0,
            elapsed_activated: Duration::ZERO,
        }
    }
}

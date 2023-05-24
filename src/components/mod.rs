use std::time::Duration;

use bevy::prelude::*;

// Define a struct to keep some information about our entity.
// Here it's an arbitrary movement speed, the spawn location, and a maximum distance from it.
#[derive(Component)]
pub struct Movable {
    pub spawn: Vec3,
    pub max_distance: f32,
    pub speed: f32,
}

// Implement a utility function for easier Movable struct creation.
impl Movable {
    pub fn new(spawn: Vec3) -> Self {
        Movable {
            spawn,
            max_distance: 5.0,
            speed: 2.0,
        }
    }
}

#[derive(Component)]
pub struct ProximityActivated {
    pub is_activated: bool,
    pub time_till_deactivated: Duration,
}

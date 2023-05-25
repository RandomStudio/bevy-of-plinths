use std::time::Duration;

use bevy::prelude::*;

#[derive(Component)]
pub struct MovablePerson {
    pub forward_speed: f32,
    pub opposing_force: Vec3,
}

impl MovablePerson {
    pub fn new() -> Self {
        MovablePerson {
            forward_speed: 0.,
            opposing_force: Vec3::ZERO,
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
            detection_radius: 2.0,
            elapsed_activated: Duration::ZERO,
        }
    }
}

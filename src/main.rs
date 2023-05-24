use bevy::prelude::*;
use systems::{camera_control, light_up_activated, make_close_activated, move_people, setup_scene};

mod components;
mod systems;

use std::time::Duration;

fn main() {
    assert_eq!(
        Duration::new(0, 1).checked_sub(Duration::new(0, 0)),
        Some(Duration::new(0, 1))
    );
    assert_eq!(
        Duration::new(4, 0).checked_sub(Duration::new(1, 0)),
        Some(Duration::new(3, 0))
    );
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_scene)
        .add_system(camera_control)
        .add_system(move_people)
        .add_system(make_close_activated)
        .add_system(light_up_activated)
        .add_system(bevy::window::close_on_esc)
        .run();
}

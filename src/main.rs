use bevy::prelude::*;
use systems::{control_camera, light_up_activated, make_close_activated, move_person};

mod components;
mod systems;
mod utils;

use std::time::Duration;

use crate::systems::{control_person, scene::setup_scene};

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
        .insert_resource(ClearColor(Color::rgb(0.2, 0., 0.1)))
        .add_startup_system(setup_scene)
        .add_system(control_camera)
        .add_system(control_person)
        .add_system(move_person)
        .add_system(make_close_activated)
        .add_system(light_up_activated)
        .add_system(bevy::window::close_on_esc)
        .run();
}

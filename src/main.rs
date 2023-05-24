use bevy::prelude::*;
use systems::{camera_control_system, move_people, setup_scene};

mod components;
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_scene)
        .add_system(camera_control_system)
        .add_system(move_people)
        .add_system(bevy::window::close_on_esc)
        .run();
}

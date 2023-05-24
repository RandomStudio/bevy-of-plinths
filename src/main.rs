use bevy::prelude::*;
use systems::{camera_control, light_up_activated, make_close_activated, move_people, setup_scene};

mod components;
mod systems;

fn main() {
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

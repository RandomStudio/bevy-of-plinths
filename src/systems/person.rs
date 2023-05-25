use bevy::prelude::*;

use crate::components::{MovablePerson, ProximityActivated};

use super::fixtures::BOX_WIDTH;

pub fn move_person(mut people: Query<(&mut Transform, &mut MovablePerson)>, time: Res<Time>) {
    // TODO: this assumes multiple people
    for (mut transform, mut person) in &mut people {
        // // Check if the entity moved too far from its spawn, if so invert the moving direction.
        // if (person.spawn - transform.translation).length() > person.max_distance {
        //     person.speed *= -1.0;
        // }
        let direction = transform.local_z();
        transform.translation +=
            direction * person.forward_speed * time.delta_seconds() + person.opposing_force;

        // Also decay opposing force
        let current_force = person.opposing_force;
        person.opposing_force -= current_force * time.delta_seconds() * 0.25;

        // Also gradually decay forward speed
        if person.forward_speed > 0. {
            person.forward_speed -= time.delta_seconds() * 0.05;
        }
    }
}

pub fn control_person(
    mut people: Query<(&mut Transform, &mut MovablePerson)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut transform, mut person) = people.single_mut();
    if input.pressed(KeyCode::W) {
        person.forward_speed += time.delta_seconds();
    }
    if input.pressed(KeyCode::S) {
        person.forward_speed -= time.delta_seconds();
    }
    if input.pressed(KeyCode::A) {
        transform.rotate_y(time.delta_seconds() * person.forward_speed.max(1.0));
    }
    if input.pressed(KeyCode::D) {
        transform.rotate_y(-time.delta_seconds() * person.forward_speed.max(1.0));
    }
}

pub fn collision_detection(
    fixtures: Query<&GlobalTransform, With<ProximityActivated>>,
    mut user: Query<(&GlobalTransform, &mut MovablePerson)>,
    time: Res<Time>,
) {
    let (user_transform, mut person) = user.single_mut();
    let user_xz = Vec2::new(
        user_transform.translation().x,
        user_transform.translation().z,
    );

    for fixture_transform in &fixtures {
        let fixture_xz = Vec2::new(
            fixture_transform.translation().x,
            fixture_transform.translation().z,
        );
        let delta = user_xz - fixture_xz;
        let distance = delta.length().abs();
        if distance < BOX_WIDTH * 1.25 {
            println!(
                "collision! {:?} to {:?} is {}",
                fixture_transform.translation(),
                user_transform.translation(),
                distance
            );
            person.opposing_force = Vec3::new(delta.x, 0., delta.y)
                * time.delta().as_secs_f32()
                * person.forward_speed.max(1.0);
            // person.forward_speed -= time.delta().as_secs_f32() / 2.0;
            // person.forward_speed = person.forward_speed.max(0.);
        }
    }
}

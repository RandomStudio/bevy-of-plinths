use std::time::Duration;

use bevy::prelude::*;

use crate::components::{MovablePerson, ProximityActivated};

pub mod scene;

pub fn move_person(mut people: Query<(&mut Transform, &MovablePerson)>, timer: Res<Time>) {
    // TODO: this assumes multiple people
    for (mut transform, person) in &mut people {
        // // Check if the entity moved too far from its spawn, if so invert the moving direction.
        // if (person.spawn - transform.translation).length() > person.max_distance {
        //     person.speed *= -1.0;
        // }
        let direction = transform.local_z();
        transform.translation += direction * person.forward_speed * timer.delta_seconds();
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

pub fn control_camera(
    mut camera: Query<(&mut Camera, &mut Transform, &GlobalTransform), With<Camera3d>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (_camera, mut camera_transform, _camera_global_transform) = camera.single_mut();
    let rotation = if input.pressed(KeyCode::Left) {
        time.delta_seconds()
    } else if input.pressed(KeyCode::Right) {
        -time.delta_seconds()
    } else {
        0.0
    };

    let movement = if input.pressed(KeyCode::Up) {
        -time.delta_seconds()
    } else if input.pressed(KeyCode::Down) {
        time.delta_seconds()
    } else {
        0.0
    };
    let direction = -camera_transform.local_y();
    camera_transform.translation += direction * movement;

    camera_transform.rotate_around(
        Vec3::ZERO,
        Quat::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
    );

    camera_transform.look_at(Vec3::ZERO, Vec3::Y);
}

const LIT: f32 = 4.0;
const DIMMED: f32 = 0.1;
const TIME_TILL_DEACTIVATED: Duration = Duration::from_millis(10000);

pub fn light_up_activated(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut entities: Query<(&Handle<StandardMaterial>, &mut ProximityActivated)>,
    time: Res<Time>,
) {
    for (material_handle, mut proximity) in &mut entities {
        let material = materials.get_mut(material_handle).unwrap();
        if proximity.is_activated {
            proximity.elapsed_activated += time.delta();
            if proximity.elapsed_activated >= TIME_TILL_DEACTIVATED {
                println!(
                    "Deactivated B, {} >= {}",
                    proximity.elapsed_activated.as_millis(),
                    TIME_TILL_DEACTIVATED.as_millis()
                );
                proximity.is_activated = false;
            } else {
                let total_time = TIME_TILL_DEACTIVATED.as_secs_f32();
                let progress = proximity.elapsed_activated.as_secs_f32() / total_time;
                // println!("{} / {} = {}", time_left, total_time, percentage);
                let brightness = (1.0 - progress) * LIT.max(DIMMED);
                material.emissive.set_r(brightness);
                material.emissive.set_g(brightness);
                material.emissive.set_b(brightness);
            }
        } else {
            material.emissive.set_r(DIMMED);
            material.emissive.set_g(DIMMED);
            material.emissive.set_b(DIMMED);
        }
    }
    // for (mut fixture, proximity) in &mut fixtures {}
}

pub fn make_close_activated(
    mut fixtures: Query<(&GlobalTransform, &mut ProximityActivated)>,
    user: Query<&GlobalTransform, With<MovablePerson>>,
) {
    // TODO: this works for single user
    let user_transform = user.single();

    for (transform, mut proximity) in &mut fixtures {
        let delta = transform.translation() - user_transform.translation();
        let distance = delta.length().abs();
        if !proximity.is_activated && distance < proximity.detection_radius {
            proximity.is_activated = true;
            proximity.elapsed_activated = Duration::ZERO;
            println!("Activated! {}", proximity.elapsed_activated.as_millis());
        }
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
        if distance < 1.0 {
            // println!(
            //     "collision? {:?} to {:?} is {}",
            //     fixture_transform.translation(),
            //     user_transform.translation(),
            //     distance
            // );
            person.forward_speed -= time.delta().as_secs_f32() / 2.0;
            person.forward_speed = person.forward_speed.max(0.);
        }
    }
}

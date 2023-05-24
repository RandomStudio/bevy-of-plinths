use std::time::Duration;

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

use crate::components::{Movable, ProximityActivated};

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PointLightBundle {
        // transform: Transform::from_xyz(5.0, 8.0, 2.0),
        transform: Transform::from_xyz(0., 5., 0.),
        point_light: PointLight {
            intensity: 1000.0, // lumens
            color: Color::WHITE,

            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(20.0).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::GRAY,
            perceptual_roughness: 0.9,
            ..default()
        }),
        ..default()
    });

    // 3D camera
    commands.spawn((
        Camera3dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

    let brightness: f32 = 0.1;

    // let material_emissive = materials.add(StandardMaterial {
    //     emissive: Color::Hsla {
    //         hue: 0.,
    //         saturation: 0.,
    //         lightness: brightness,
    //         alpha: 1.0,
    //     },
    //     // Color::rgb_linear(brightness, brightness, brightness), // 4. Put something bright in a dark environment to see the effect
    //     ..default()
    // });

    let size = 0.2;
    let spacing = 10.0;

    let light_box_mesh = meshes.add(
        shape::Box {
            min_x: -size,
            max_x: size,
            min_y: 0.,
            max_y: size * 4.,
            min_z: -size,
            max_z: size,
        }
        // shape::Cube { size: 0.5 }
        // shape::Icosphere {
        //     radius: 0.5,
        //     subdivisions: 5,
        // }
        .try_into()
        .unwrap(),
    );

    for row in -4..4 {
        for col in -4..4 {
            let x = row as f32 * size * spacing;
            let y = 0.0;
            let z = col as f32 * size * spacing;
            commands.spawn((
                PbrBundle {
                    mesh: light_box_mesh.clone(),
                    material: materials.add(StandardMaterial {
                        emissive: Color::Hsla {
                            hue: 0.,
                            saturation: 0.,
                            lightness: brightness,
                            alpha: 1.0,
                        },
                        // Color::rgb_linear(brightness, brightness, brightness), // 4. Put something bright in a dark environment to see the effect
                        ..default()
                    }),
                    transform: Transform::from_xyz(x, y, z),
                    ..default()
                },
                ProximityActivated {
                    is_activated: false,
                    elapsed_activated: Duration::ZERO,
                },
            ));

            // Adding too many lights appears to break the lighting system -
            // might need to use baked lighting or cheat using vertex
            // colours on a floor grid

            // commands.spawn(PointLightBundle {
            //     // transform: Transform::from_xyz(5.0, 8.0, 2.0),
            //     transform: Transform::from_xyz(x, y + 2.0, x),
            //     point_light: PointLight {
            //         intensity: 60.0, // lumens - roughly a 100W non-halogen incandescent bulb
            //         color: if row % 2 == 0 {
            //             Color::RED
            //         } else {
            //             Color::BLUE
            //         },
            //         range: 5.0,
            //         shadows_enabled: false,

            //         ..default()
            //     },
            //     ..default()
            // });
        }
    }

    let entity_spawn = Vec3::ZERO;

    let person = meshes.add(
        shape::Icosphere {
            radius: size,
            subdivisions: 5,
        }
        .try_into()
        .unwrap(),
    );

    commands.spawn((
        PbrBundle {
            mesh: person.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                ..default()
            }),
            transform: Transform::from_translation(entity_spawn).with_translation(Vec3::new(
                0.,
                size,
                size * spacing / 2.,
            )),
            ..default()
        },
        Movable::new(entity_spawn),
    ));
}

// This system will move all Movable entities with a Transform
pub fn move_people(mut people: Query<(&mut Transform, &mut Movable)>, timer: Res<Time>) {
    for (mut transform, mut person) in &mut people {
        // Check if the entity moved too far from its spawn, if so invert the moving direction.
        if (person.spawn - transform.translation).length() > person.max_distance {
            person.speed *= -1.0;
        }
        let direction = transform.local_x();
        transform.translation += direction * person.speed * timer.delta_seconds();
    }
}

pub fn camera_control(
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
        time.delta_seconds()
    } else if input.pressed(KeyCode::Down) {
        -time.delta_seconds()
    } else {
        0.0
    };
    let direction = camera_transform.local_y();
    camera_transform.translation += direction * movement;

    camera_transform.rotate_around(
        Vec3::ZERO,
        Quat::from_euler(EulerRot::XYZ, 0.0, rotation, 0.0),
    );

    camera_transform.look_at(Vec3::ZERO, Vec3::Y);
}

const LIT: f32 = 4.0;
const DIMMED: f32 = 0.1;
const TIME_TILL_DEACTIVATED: Duration = Duration::from_millis(3000);

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
    user: Query<&GlobalTransform, With<Movable>>,
) {
    // TODO: this works for single user
    let user_transform = user.single();

    for (transform, mut proximity) in &mut fixtures {
        let delta = transform.translation() - user_transform.translation();
        let distance = delta.length().abs();
        if !proximity.is_activated && distance < 2.0 {
            proximity.is_activated = true;
            proximity.elapsed_activated = Duration::ZERO;
            println!("Activated! {}", proximity.elapsed_activated.as_millis());
        }
    }
}

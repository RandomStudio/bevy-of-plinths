use std::time::Duration;

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

use crate::{
    components::{Movable, ProximityActivated},
    utils::map_range,
};

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

    let brightness: f32 = 0.1;

    let box_height = 1.25;
    let box_width = 0.5;
    let spacing = 2.0; // centre-to-centre

    let light_box_mesh = meshes.add(
        shape::Box {
            min_x: -box_width / 2.0,
            max_x: box_width / 2.0,
            min_y: 0.,
            max_y: box_height,
            min_z: -box_width / 2.0,
            max_z: box_width / 2.0,
        }
        .try_into()
        .unwrap(),
    );

    let num_rows: i32 = 4;
    let num_cols: i32 = 4;

    let row_max = num_rows as f32 - 1.;
    let col_max = num_cols as f32 - 1.;

    for row in 0..num_rows {
        for col in 0..num_cols {
            let x = map_range(
                row as f32,
                0.,
                row_max,
                -spacing * row_max / 2.0,
                spacing * row_max / 2.0,
            );
            let y = 0.0;
            let z = map_range(
                col as f32,
                0.,
                col_max,
                -spacing * col_max / 2.0,
                spacing * col_max / 2.0,
            );
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
                ProximityActivated::new(),
            ));
        }
    }

    // ground plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(row_max * spacing * 3.0).into()),
        material: materials.add(StandardMaterial {
            base_color: Color::GRAY,
            perceptual_roughness: 0.5,
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
            transform: Transform::from_xyz(0., 2.5, row_max * spacing)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        BloomSettings::default(), // 3. Enable bloom for the camera
    ));

    let person_radius = 0.25;
    let person_height = 1.8;

    let body = meshes.add(
        shape::Capsule {
            radius: person_radius,
            depth: person_height,
            ..default()
        }
        // shape::Icosphere {
        //     radius: size,
        //     subdivisions: 5,
        // }
        .try_into()
        .unwrap(),
    );

    let face = meshes.add(
        shape::Cylinder {
            radius: person_radius,
            height: person_radius / 4.0,
            ..default()
        }
        .try_into()
        .unwrap(),
    );

    commands
        .spawn((
            PbrBundle {
                mesh: body,
                material: materials.add(StandardMaterial {
                    base_color: Color::BLUE,
                    ..default()
                }),
                transform: Transform::default().with_translation(Vec3::new(
                    0.,
                    (person_height + person_radius * 2.0) / 2.0,
                    0.,
                )),
                ..default()
            },
            Movable::new(),
        ))
        .with_children(|builder| {
            builder.spawn(PbrBundle {
                mesh: face,
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    ..default()
                }),
                transform: Transform::default()
                    .with_translation(Vec3::new(
                        0.,
                        person_height / 2.0 - person_radius,
                        person_radius,
                    ))
                    .with_rotation(Quat::from_euler(
                        EulerRot::XYZ,
                        f32::to_radians(90.),
                        0.,
                        0.0,
                    )),
                ..default()
            });
        });
}

pub fn move_person(mut people: Query<(&mut Transform, &Movable)>, timer: Res<Time>) {
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
    mut people: Query<(&mut Transform, &mut Movable)>,
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
        transform.rotate_y(time.delta_seconds());
    }
    if input.pressed(KeyCode::D) {
        transform.rotate_y(-time.delta_seconds());
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
        -time.delta_seconds() // closer
    } else if input.pressed(KeyCode::Down) {
        time.delta_seconds() // further back
    } else {
        0.0
    };
    let direction = camera_transform.local_z();
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
        if !proximity.is_activated && distance < proximity.detection_radius {
            proximity.is_activated = true;
            proximity.elapsed_activated = Duration::ZERO;
            println!("Activated! {}", proximity.elapsed_activated.as_millis());
        }
    }
}

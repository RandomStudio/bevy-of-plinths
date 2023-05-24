use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

use crate::{
    components::{Movable, ProximityActivated},
    utils::map_range,
};

const NUM_ROWS: i32 = 5;
const NUM_COLS: i32 = 5;

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

    let row_max = NUM_ROWS as f32 - 1.;
    let col_max = NUM_COLS as f32 - 1.;

    for row in 0..NUM_ROWS {
        for col in 0..NUM_COLS {
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
            transform: Transform::from_xyz(0., 2.5, -row_max * spacing)
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

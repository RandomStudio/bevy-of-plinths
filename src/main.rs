use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
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

    let material_emissive1 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(13.99, 5.32, 2.0), // 4. Put something bright in a dark environment to see the effect
        ..default()
    });
    let material_emissive2 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(2.0, 13.99, 5.32),
        ..default()
    });
    let material_emissive3 = materials.add(StandardMaterial {
        emissive: Color::rgb_linear(5.32, 2.0, 13.99),
        ..default()
    });
    let material_non_emissive = materials.add(StandardMaterial {
        base_color: Color::GRAY,
        ..default()
    });

    let mesh = meshes.add(
        shape::Icosphere {
            radius: 0.5,
            subdivisions: 5,
        }
        .try_into()
        .unwrap(),
    );

    for x in -10..10 {
        for z in -10..10 {
            let mut hasher = DefaultHasher::new();
            (x, z).hash(&mut hasher);
            let rand = (hasher.finish() - 2) % 6;

            let material = match rand {
                0 => material_emissive1.clone(),
                1 => material_emissive2.clone(),
                2 => material_emissive3.clone(),
                3 | 4 | 5 => material_non_emissive.clone(),
                _ => unreachable!(),
            };

            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                material,
                transform: Transform::from_xyz(x as f32 * 2.0, 0.0, z as f32 * 2.0),
                ..default()
            });
        }
    }
}

use bevy::{
    core_pipeline::{bloom::BloomSettings, tonemapping::Tonemapping},
    prelude::*,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_scene)
        .add_system(camera_control_system)
        .add_system(bevy::window::close_on_esc)
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

    let brightness: f32 = 4.0;

    let material_emissive = materials.add(StandardMaterial {
        emissive: Color::Hsla {
            hue: 0.,
            saturation: 0.,
            lightness: brightness,
            alpha: 1.0,
        },
        // Color::rgb_linear(brightness, brightness, brightness), // 4. Put something bright in a dark environment to see the effect
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

    for x in -4..4 {
        for z in -4..4 {
            commands.spawn(PbrBundle {
                mesh: mesh.clone(),
                material: material_emissive.clone(),
                transform: Transform::from_xyz(x as f32 * 2.0, 0.0, z as f32 * 2.0),
                ..default()
            });
        }
    }
}

fn camera_control_system(
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

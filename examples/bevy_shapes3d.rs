use bevy::color::palettes::css::{BLACK, BLUE};
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (rotate_shapes, move_camera))
        .run();
}

// Marker for shapes we want to rotate
#[derive(Component)]
struct RotatingShape {
    index: usize,
    base_pos: Vec3,
}

// Marker for the camera so we can move it
#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 1. LIGHTING
    // Note: Standard Bevy shadows are expensive.
    // Set `shadows_enabled: false` to match your optimized Breeze version.
    commands.spawn((
        DirectionalLight {
            shadows_enabled: false, // Toggle this to compare!
            illuminance: 10_000.0,
            ..default()
        },
        Transform::from_xyz(-0.5, -1.0, -0.5).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // 2. CAMERA
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 4.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
        MainCamera,
    ));

    // 3. FLOOR (Plane)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.2, 0.2, 0.2))), // Dark Gray
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));

    // 4. SHAPES
    let spacing = 3.0;

    // Cube (Index 0, Offset 0.0)
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::from_length(1.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 0.0, 1.0))), // Blue
        Transform::from_xyz(0.0, 0.5, 0.0),
        RotatingShape { index: 0, base_pos: Vec3::new(0.0, 0.5, 0.0) },
    ));

    // Sphere (Index 1, Offset 1.0) - Note: Spheres usually don't show rotation well
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 0.0))), // Red
        Transform::from_xyz(spacing, 0.5, 0.0),
        RotatingShape { index: 1, base_pos: Vec3::new(spacing, 0.5, 0.0) },
    ));

    // Cylinder (Index 2, Offset 2.0)
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(0.5, 2.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.0, 1.0, 0.0))), // Lime
        Transform::from_xyz(spacing * 2.0, 1.0, 0.0),
        RotatingShape { index: 2, base_pos: Vec3::new(spacing * 2.0, 1.0, 0.0) },
    ));

    // Torus (Index 3, Offset 3.0)
    commands.spawn((
        Mesh3d(meshes.add(Torus::new(0.6, 0.2))),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 1.0))), // Fuchsia
        Transform::from_xyz(spacing * 3.0, 0.5, 0.0),
        RotatingShape { index: 3, base_pos: Vec3::new(spacing * 3.0, 0.5, 0.0) },
    ));

    // Cone (Index 4, Offset 4.0)
    commands.spawn((
        Mesh3d(meshes.add(Cone { radius: 1.0, height: 2.0 })),
        MeshMaterial3d(materials.add(Color::srgb(1.0, 0.65, 0.0))), // Orange
        Transform::from_xyz(spacing * 4.0, 1.0, 0.0),
        RotatingShape { index: 4, base_pos: Vec3::new(spacing * 4.0, 1.0, 0.0) },
    ));
}

fn rotate_shapes(time: Res<Time>, mut query: Query<(&mut Transform, &RotatingShape)>) {
    let t = time.elapsed_secs();

    // Logic matching your ShapeGallery offsets: [0.0, 1.0, 2.0, 3.0, 4.0]
    for (mut transform, shape) in &mut query {
        let offset = shape.index as f32; // Simplified offset logic

        let rot = Quat::from_rotation_y(t + offset) * Quat::from_rotation_x((t + offset) * 0.5);

        transform.rotation = rot;
    }
}

fn move_camera(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<MainCamera>>
) {
    let speed = 10.0;
    let dt = time.delta_secs();
    let mut x_move = 0.0;

    if keyboard.pressed(KeyCode::ArrowLeft) {
        x_move -= speed * dt;
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        x_move += speed * dt;
    }

    for mut transform in &mut query {
        // Move camera position
        transform.translation.x += x_move;

        // Keep looking at target (x, 0, 0)
        let target = Vec3::new(transform.translation.x, 0.0, 0.0);
        transform.look_at(target, Vec3::Y);
    }
}
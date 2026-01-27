use breeze::prelude::*;

struct OrbitingLight;

impl Scene for OrbitingLight {

    fn draw(&mut self, ctx: &mut DrawContext) {
        // Clear background to dark color so lighting stands out
        ctx.clear_background(Color::from(BLACK));

        ctx.with_layer(0, |world| {

            // Setup Camera
            // Positioned up and back, looking at the center
            world.set_camera(CameraMode::Camera3d {
                position: Vec3::new(0.0, 6.0, 12.0),
                target: Vec3::ZERO,
            });

            // Draw the Central Cube
            // We use White to best reflect the color of the light
            let time = ctx.time.elapsed_secs();
            world.draw3d.cube(
                Vec3::ZERO,
                Quat::from_rotation_y(time) * Quat::from_rotation_x((time) * 0.5),
                2.0,
                None,
                Color::from(WHITE),
            );

            // Calculate Orbiting Light Position
            let time = ctx.time.elapsed_secs();
            let orbit_radius = 6.0;
            let speed = 2.0;

            let light_pos = Vec3::new(
                (time * speed).cos() * orbit_radius,
                2.0, // Hover 2 units above the floor
                (time * speed).sin() * orbit_radius
            );

            // 6. Draw a small sphere to visualize the light source itself
            world.draw3d.sphere(
                light_pos,
                0.2,
                None,
                Color::from(YELLOW)
            );

            // Create the actual Point Light
            // Parameters: position, color, intensity, radius (range). shadows disabled
            world.lights.point(
                light_pos,
                Color::from(YELLOW),
                100_000.0,
                20.0,
                false
            );
        });

    }
}

fn main() {
    Breeze::default()
        .title("Orbiting Light Demo")
        .resolution(1280, 720)
        .run(OrbitingLight);
}
use breeze::prelude::*;

struct OrbitingLight;

impl Game for OrbitingLight {
    fn update(&mut self, _ctx: &mut Context) {
        // No input logic required for this demo
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        // 1. Clear background to dark color so lighting stands out
        ctx.clear_background(Color::from(BLACK));

        ctx.with_layer(0, |world| {
            // 2. Setup Camera
            // Positioned up and back, looking at the center
            world.set_camera(CameraMode::Camera3d {
                position: Vec3::new(0.0, 6.0, 12.0),
                target: Vec3::ZERO,
            });

            // 3. Draw a Floor (to catch shadows/lighting)
            world.draw3d.plane(
                Vec3::new(0.0, -1.0, 0.0),
                Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2),
                20.0,
                Color::from(DARK_GRAY)
            );

            // 4. Draw the Central Cube
            // We use White to best reflect the color of the light
            world.draw3d.cube(
                Vec3::ZERO,
                Quat::from_rotation_x(0.5) * Quat::from_rotation_y(0.78),
                2.0,
                Color::from(WHITE),
            );

            // 5. Calculate Orbiting Light Position
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
                Color::from(YELLOW)
            );

            // 7. Create the actual Point Light
            // Parameters: position, color, intensity, radius (range). shadows disabled
            world.lights.point(
                light_pos,
                Color::from(YELLOW),
                1_000_000.0,
                20.0,
                false
            );
        });

        // UI Layer: Show FPS
        ctx.with_layer(1, |ui| {
            ui.set_camera(CameraMode::default());
            //ui.draw_fps(vec2(20.0, 20.0), Color::WHITE);
        });
    }
}

fn main() {
    Breeze::default()
        .title("Orbiting Light Demo")
        .resolution(1280, 720)
        .run(OrbitingLight);
}
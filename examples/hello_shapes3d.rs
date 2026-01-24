use breeze::prelude::*;
use breeze::TextureAsset;

struct ShapeGallery {
    camera_x: f32,
    offsets: Vec<f32>,
    texture1: Option<TextureAsset>,
}

impl Scene for ShapeGallery {

    fn init(&mut self, ctx: &mut Context) {
        self.texture1 = Some(ctx.asset_server.load("breeze.png"));
    }

    fn update(&mut self, ctx: &mut Context) -> SceneTransition {
        let speed = 10.0;
        let dt = ctx.time.delta_secs();

        if ctx.input.key_down(KeyCode::ArrowLeft) {
            self.camera_x -= speed * dt;
        }
        if ctx.input.key_down(KeyCode::ArrowRight) {
            self.camera_x += speed * dt;
        }
        
        SceneTransition::None
    }

    fn draw(&mut self, ctx: &mut DrawContext) {

        // 2. Clear background
        ctx.clear_background(Color::from(BLACK));

        ctx.with_layer(0, |world| {

            // Setup lighting
            world.lights.directional(
                Vec3::new(-0.5, -1.0, -0.5),
                Color::from(WHITE),
                10_000.0,
                false
            );

            // We position the camera back (z=8) and up (y=4), looking at the current x position
            world.set_camera(CameraMode::Camera3d {
                position: Vec3::new(self.camera_x, 4.0, 8.0),
                target: Vec3::new(self.camera_x, 0.0, 0.0),
            });

            // Draw a Row of Shapes
            // We'll use a helper variable for spacing
            let spacing = 3.0;
            let time = ctx.time.elapsed_secs();

            let rot = |i: usize| -> Quat {
                // Use the stored offset for this index (fallback to 0.0 if missing)
                let offset = *self.offsets.get(i).unwrap_or(&0.0);

                // Your requested calculation + offset
                Quat::from_rotation_y(time + offset) * Quat::from_rotation_x((time + offset) * 0.5)
            };

            // x = 0: Cube
            world.draw3d.cube(
                Vec3::new(0.0, 0.5, 0.0),
                rot(1),
                1.0,
                self.texture1.clone(),
                WHITE.into()
            );

            // x = 3: Sphere
            world.draw3d.sphere(
                Vec3::new(spacing, 0.5, 0.0),
                0.5,
                None,
                Color::from(RED)
            );

            // x = 6: Cylinder
            world.draw3d.cylinder(
                Vec3::new(spacing * 2.0, 1.0, 0.0),
                rot(2),
                0.5,
                2.0,
                None,
                Color::from(LIME)
            );

            // x = 12: Torus
            world.draw3d.torus(
                Vec3::new(spacing * 3.0, 0.5, 0.0),
                rot(3),
                0.6,
                0.2,
                None,
                Color::from(FUCHSIA)
            );

            // x = 15: Cone
            world.draw3d.cone(
                Vec3::new(spacing * 4.0, 1.0, 0.0),
                rot(4),
                1.0,
                2.0,
                None,
                Color::from(ORANGE)
            );

            // 6. Draw Text Instructions (in world space or screen space)
            // Note: Currently text is 2D screen space.
            // It will stick to the screen even as 3D moves, which is perfect for UI.
            world.text.draw("Use Left/Right Arrow Keys to View Shapes", vec2(-200.0, 250.0));
        });

        // UI Layer to show fps
        ctx.with_layer(1, |ui| {
            ui.set_camera(CameraMode::default());
            ui.draw_fps(vec2(300.0, 250.0), Color::BLACK);
        });
    }
}

fn main() {
    Breeze::default()
        .title("Breeze example")
        .resolution(800, 600)
        .run(ShapeGallery {
            camera_x: 0.0,
            offsets: vec![0.0, 1.0, 2.0, 3.0, 4.0],
            texture1: None,
        });
}
use breeze::prelude::*;

struct MyGame {
    my_model: Handle<Scene>,
}

impl Game for MyGame {
    fn init(&mut self, ctx: &mut Context) {
        self.my_model = ctx.load_scene("models/Fox.gltf#Scene0");
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        ctx.clear_background(Color::from(LIGHT_SKY_BLUE));

        ctx.with_layer(0, |world| {

            // Set up a directional light
            world.lights.directional(
                Vec3::new(-0.5, -1.0, -0.5),
                Color::from(WHITE),
                10_000.0,
                false
            );

            world.set_camera(CameraMode::Camera3d {
                position: Vec3::new(0.0, 100.0, 200.0),
                target: Vec3::ZERO,
            });

            world.draw3d.model(
                Vec3::new(0.0, 0.0, 0.0),
                Quat::from_rotation_y(ctx.time.elapsed_secs()),
                Vec3::splat(1.0),
                self.my_model.clone()
            );
        });
    }
}

fn main() {
    run(AppConfig {
        title: "Breeze - 3D Avocado".into(),
        ..default()
    }, MyGame {
        my_model: Handle::default(),
    });
}
use breeze::prelude::*;

struct MyGame {
    camera_pos: Vec2,
    camera_scale: f32,
}

impl Scene for MyGame {

    fn update(&mut self, ctx: &mut Context) -> SceneTransition {

        let speed = 400.0 * ctx.time.delta_secs();

        // Movement
        if ctx.input.key_down(KeyCode::ArrowRight) {
            self.camera_pos.x += speed;
        }
        if ctx.input.key_down(KeyCode::ArrowLeft) {
            self.camera_pos.x -= speed;
        }
        if ctx.input.key_down(KeyCode::ArrowUp) {
            self.camera_pos.y += speed;
        }
        if ctx.input.key_down(KeyCode::ArrowDown) {
            self.camera_pos.y -= speed;
        }

        // Zoom (Z to Zoom Out, X to Zoom In)
        if ctx.input.key_down(KeyCode::KeyZ) {
            self.camera_scale *= 1.02;
        }
        if ctx.input.key_down(KeyCode::KeyX) {
            self.camera_scale /= 1.02;
        }

        SceneTransition::None
    }

    fn draw(&mut self, ctx: &mut DrawContext) {

        ctx.clear_background(Color::from(DARK_SLATE_GRAY));

        ctx.with_layer(1, |ui| {
            ui.set_camera(CameraMode::default());
            ui.text.draw("Use Arrow Keys to Move Camera, Z/X to Zoom In/Out", vec2(0.0, 250.0));
        });

        ctx.with_layer(0, |world2d| {

            let x_pos = ctx.time.elapsed_secs().sin() * 200.0;

            world2d.set_camera(CameraMode::Camera2d {
                position: self.camera_pos,
                scale: self.camera_scale,
            });

            world2d.draw2d.circle(vec2(x_pos, 0.0), 60.0, None, Color::from(RED));

            // Draw a static Rectangle
            world2d.draw2d.rect(vec2(0.0, -200.0), vec2(400.0, 50.0), None, Color::from(BLUE_VIOLET));
        });

    }
}

fn main() {
    Breeze::default()
        .title("Hello, Camera2D!")
        .resolution(800, 600)
        .run(MyGame {
            camera_pos: Vec2::ZERO,
            camera_scale: 1.0,
        });
}
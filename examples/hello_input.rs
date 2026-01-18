use breeze::prelude::*;

struct MyGame {
    pos: Vec2,
    color: Color,
}

impl Game for MyGame {
    fn update(&mut self, ctx: &mut Context) {

        let speed = 200.0 * ctx.time.delta_secs();

        if ctx.input.key_down(KeyCode::ArrowUp) { self.pos.y += speed; }
        if ctx.input.key_down(KeyCode::ArrowDown) { self.pos.y -= speed; }
        if ctx.input.key_down(KeyCode::ArrowRight) { self.pos.x += speed; }
        if ctx.input.key_down(KeyCode::ArrowLeft) { self.pos.x -= speed; }

        if ctx.input.mouse_pressed(MouseButton::Left) {
            self.pos = ctx.input.mouse_pos();
            self.color = Color::srgb(0.0, 1.0, 0.0); // Turn Green
        } else if ctx.input.mouse_pressed(MouseButton::Right) {
            self.pos = ctx.input.mouse_pos();
            self.color = Color::srgb(1.0, 0.0, 0.0); // Revert to Red
        }

    }

    fn draw(&mut self, ctx: &mut DrawContext) {

        // Clear background
        ctx.clear_background(Color::from(LIGHT_SALMON));

        // World layer
        ctx.with_layer(0, |world2d| {
            world2d.set_camera(CameraMode::default());
            world2d.draw2d.circle(vec2(self.pos.x, self.pos.y), 30.0, self.color);
        });

        // UI layer
        ctx.with_layer(1, |ui| {
            ui.set_camera(CameraMode::default());
            ui.text.draw("Use Arrow Keys to Move Circle. Left Click to turn Green, Right Click to turn Red.", vec2(-0.0, 250.0),
            );
        });
    }
}

fn main() {
    Breeze::default()
        .title("Hello, Input!")
        .resolution(800, 600)
        .run(MyGame {
            pos: Vec2::ZERO,
            color: Color::srgb(1.0, 0.0, 0.0), // Start as Red
        });
}
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
        ctx.shapes.circle(self.pos.x, self.pos.y, 30.0, self.color);
    }
}

fn main() {
    let my_game = MyGame {
        pos: Vec2::ZERO,
        color: Color::srgb(1.0, 0.0, 0.0), // Start Red
    };

    run(AppConfig::default(), my_game);
}
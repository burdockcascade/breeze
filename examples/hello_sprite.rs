use breeze::prelude::*;

struct MyGame {
    player_pos: Vec2,
    breeze_logo: Handle<Image>,
}

impl Game for MyGame {
    fn init(&mut self, ctx: &mut Context) {
        // Load the player sprite
        self.breeze_logo = ctx.load_image("breeze.png");
    }

    fn update(&mut self, ctx: &mut Context) {
        // Move with keys
        if ctx.input.key_down(KeyCode::ArrowRight) { self.player_pos.x += 2.0; }
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        // Draw the sprite at current position
        ctx.sprites.draw_ext(&self.breeze_logo, self.player_pos.x, self.player_pos.y, 0.25, Color::WHITE);
    }
}

fn main() {
    run(AppConfig::default(), MyGame {
        player_pos: Vec2::ZERO,
        breeze_logo: Handle::default() // Start with empty handle
    });
}
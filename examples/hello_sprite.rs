use bevy::color::palettes::css;
use breeze::prelude::*;

struct MyGame {
    player_pos: Vec2,
    player_image: Handle<Image>,
}

impl Game for MyGame {
    fn init(&mut self, ctx: &mut Context) {
        // Load the player sprite
        self.player_image = ctx.load_image("bevy.png");
    }

    fn update(&mut self, ctx: &mut Context) {
        // Move with keys
        if ctx.input.key_down(KeyCode::ArrowRight) { self.player_pos.x += 2.0; }
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        // Draw the sprite at current position
        ctx.sprites.sprite(&self.player_image, self.player_pos.x, self.player_pos.y);

        // Draw a second copy, smaller and tinted blue
        ctx.sprites.sprite_ext(&self.player_image, 100.0, 100.0, 0.5, Color::from(css::BLUE));
    }
}

fn main() {
    run(AppConfig::default(), MyGame {
        player_pos: Vec2::ZERO,
        player_image: Handle::default() // Start with empty handle
    });
}
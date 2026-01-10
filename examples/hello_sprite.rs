use breeze::prelude::*;

struct MyGame {
    breeze_logo: Handle<Image>,
}

impl Game for MyGame {
    fn init(&mut self, ctx: &mut Context) {
        // Load the player sprite
        self.breeze_logo = ctx.load_image("breeze.png");
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        ctx.clear_background(Color::WHITE);
        ctx.sprites.draw_ext(&self.breeze_logo, 0.0, 0.0, 0.25, Color::WHITE);
    }
}

fn main() {
    run(AppConfig::default(), MyGame {
        breeze_logo: Handle::default() // Start with empty handle
    });
}
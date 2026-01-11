use breeze::prelude::*;

struct MyGame {
    breeze_logo: Handle<Image>,
}

impl Game for MyGame {
    fn init(&mut self, ctx: &mut Context) {
        // Load the logo sprite
        self.breeze_logo = ctx.load_image("breeze.png");
    }

    fn draw(&mut self, ctx: &mut DrawContext) {

        // Clear the screen with a white background
        ctx.clear_background(Color::WHITE);

        ctx.with_layer(1, |ui| {
            ui.set_camera(CameraMode::default());
            ui.text.draw("Welcome to Breeze!", -100.0, 150.0);
        });

        ctx.with_layer(0, |world| {

            // Set up a default camera for thw world
            world.set_camera(CameraMode::default());

            // Draw the breeze logo sprite at (0,0) with a scale of 0.25
            world.sprites.draw_ext(&self.breeze_logo, 0.0, 0.0, 0.25, Color::WHITE);

        });

    }
}

fn main() {
    run(AppConfig::default(), MyGame {
        breeze_logo: Handle::default() // Start with empty handle
    });
}
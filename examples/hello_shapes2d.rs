use breeze::prelude::*;

struct MyGame;

impl Game for MyGame {
    fn draw(&mut self, ctx: &mut DrawContext) {

        // Set background color to Dark Slate Gray
        ctx.clear_background(Color::from(LIGHT_GRAY));

        ctx.with_layer(0, |world| {

            // Set up a default camera for the world
            world.set_camera(CameraMode::default());

            // Draw some text at the top
            world.text.draw("Hello, Shapes!", vec2(-150.0, 200.0));

            // Calculate positions and sizes based on elapsed time
            let x_pos = ctx.time.elapsed_secs().sin() * 200.0;
            let ring_radius = 100.0 + (ctx.time.elapsed_secs() * 5.0).sin() * 25.0;

            // Draw a Circle that moves
            world.draw2d.circle(vec2(x_pos, 0.0), 60.0, Color::from(CORNFLOWER_BLUE));

            // Draw a static Rectangle
            world.draw2d.rect(vec2(0.0, -200.0), vec2(400.0, 50.0), Color::from(DARK_ORANGE));

            // Draw a Ring that pulses and moves with the circle
            world.draw2d.ring(vec2(x_pos, 0.0), ring_radius, 10.0, Color::from(MEDIUM_SEA_GREEN));

        });
        
    }
}

fn main() {
    run(AppConfig::default(), MyGame);
}
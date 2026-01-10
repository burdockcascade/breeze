use breeze::prelude::*;

struct MyGame;

impl Game for MyGame {
    fn init(&mut self, _ctx: &mut Context) {
    }

    fn update(&mut self, _ctx: &mut Context) {
    }

    fn draw(&mut self, ctx: &mut DrawContext) {

        // Draw a Circle that moves
        let x_pos = ctx.time.elapsed_secs().sin() * 200.0;
        ctx.shapes.circle(x_pos, 0.0, 60.0, Color::from(RED));

        // Draw a static Rectangle
        ctx.shapes.rect(0.0, -200.0, 400.0, 50.0, Color::from(BLUE_VIOLET));

        // Draw a Ring that pulses
        let ring_radius = 100.0 + (ctx.time.elapsed_secs() * 5.0).sin() * 25.0;
        ctx.shapes.ring(x_pos, 0.0, ring_radius, 10.0, Color::from(YELLOW));
    }
}

fn main() {
    run(AppConfig::default(), MyGame);
}
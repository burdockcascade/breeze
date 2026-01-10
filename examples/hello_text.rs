use breeze::prelude::*;

struct MyGame;

impl Game for MyGame {
    fn init(&mut self, _ctx: &mut Context) {
    }

    fn update(&mut self, _ctx: &mut Context) {
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        ctx.text.draw("Hello, Breeze!", -100.0, 0.0);
    }
}

fn main() {
    run(AppConfig::default(), MyGame);
}
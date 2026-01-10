use breeze::prelude::*;

struct MyGame;

impl Game for MyGame {
    fn init(&mut self, _ctx: &mut Context) {
    }

    fn update(&mut self, _ctx: &mut Context) {
    }

    fn draw(&mut self, _ctx: &mut DrawContext) {
    }
}

fn main() {
    // 1. Configure
    let config = AppConfig {
        title: "Hello Window".to_string(),
        width: 800,
        height: 600,
    };

    // 2. Run
    run(config, MyGame);
}
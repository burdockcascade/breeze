use breeze::prelude::*;

#[derive(Default)]
struct MyGame;

impl Game for MyGame {
    fn update(&mut self, ctx: &mut Context) {
        // Logic & Input
        if ctx.input.key_pressed(KeyCode::Space) {

            // Created/distributed by Kenney (www.kenney.nl)
            // License: (Creative Commons Zero, CC0)
            ctx.audio.play_vol("switch_001.ogg", 0.2);
        }
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        ctx.text.draw("Press Space", 0.0, 100.0);
    }
}

fn main() {
    run(AppConfig::default(), MyGame::default());
}
use breeze::prelude::*;

#[derive(Default)]
struct MyGame;

impl Scene for MyGame {
    fn update(&mut self, ctx: &mut Context) -> SceneTransition {
        // Logic & Input
        if ctx.input.key_pressed(KeyCode::Space) {

            // Created/distributed by Kenney (www.kenney.nl)
            // License: (Creative Commons Zero, CC0)
            ctx.audio.play_vol("switch_001.ogg", 0.2);
        }
        SceneTransition::None
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        ctx.clear_background(Color::from(DARK_SLATE_GRAY));

        ctx.with_layer(0, |ui| {
            ui.set_camera(CameraMode::default());
            ui.text.draw("Press Space to Play Sound Effect", vec2(-150.0, 0.0));
        });
    }
}

fn main() {
    Breeze::default()
        .title("Hello, Audio!")
        .resolution(800, 600)
        .run(MyGame::default());
}
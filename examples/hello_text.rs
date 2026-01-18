use breeze::prelude::*;

struct MyGame;

impl Game for MyGame {
    fn draw(&mut self, ctx: &mut DrawContext) {
        ctx.clear_background(Color::WHITE);
        ctx.with_layer(0, |ui| {
            ui.set_camera(CameraMode::default());
            ui.text.draw("Hello, Breeze!", vec2(-100.0, 0.0));
        });
    }
}

fn main() {
    Breeze::default()
        .title("Hello, Breeze!")
        .resolution(800, 600)
        .run(MyGame);
}
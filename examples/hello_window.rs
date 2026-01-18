use std::default;
use breeze::prelude::*;

struct MyGame {
    is_fullscreen: bool,
}

impl Game for MyGame {
    fn init(&mut self, _ctx: &mut Context) {
    }

    fn update(&mut self, ctx: &mut Context) {
        if ctx.input.key_pressed(KeyCode::Enter) && ctx.input.key_down(KeyCode::AltLeft) {
            self.is_fullscreen = !self.is_fullscreen;
            ctx.window.set_fullscreen(self.is_fullscreen);
        }
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        ctx.clear_background(Color::from(LIGHT_SKY_BLUE));
        ctx.with_layer(0, |ui| {
            ui.set_camera(CameraMode::default());
            if self.is_fullscreen {
                ui.text.draw("Fullscreen mode - Press Alt+Enter to toggle", Vec2::ZERO);
            } else {
                ui.text.draw("Windowed mode - Press Alt+Enter to toggle", Vec2::ZERO);
            }
        });
    }
}

fn main() {
    Breeze::default()
        .title("Breeze example")
        .resolution(640, 480)
        .run(MyGame {
            is_fullscreen: false,
        });
}
use breeze::prelude::*;

#[derive(Default)]
struct MyGame {
    music_playing: bool,
    camera: CameraMode,
}

impl Game for MyGame {
    fn init(&mut self, ctx: &mut Context) {
        // Celebration by Kamye (from looperman.com)
        ctx.audio.play_loop_vol("music", "celebrate.ogg", 0.5);
        self.music_playing = true;
    }

    fn update(&mut self, ctx: &mut Context) {
        // Pause music when space is pressed
        if ctx.input.key_pressed(KeyCode::KeyP) {
            if !self.music_playing {
                ctx.audio.resume("music");
                self.music_playing = true;
            } else {
                ctx.audio.pause("music");
                self.music_playing = false;
            }
        }

    }

    fn draw(&mut self, ctx: &mut DrawContext) {

        ctx.clear_background(Color::from(LIGHT_CORAL));

        ctx.with_layer(0, |ui| {
            ui.set_camera(self.camera);
            if self.music_playing {
                ui.text.draw("Music Playing - Press 'P' to Pause", vec2(0.0, 0.0));
            } else {
                ui.text.draw("Music Paused - Press 'P' to Resume", vec2(0.0, 0.0));
            }
        });
    }
}

fn main() {
    run(AppConfig::default(), MyGame::default());
}
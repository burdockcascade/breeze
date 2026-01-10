use breeze::prelude::*;

#[derive(Default)]
struct MyGame {
    music_playing: bool
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
        if self.music_playing {
            ctx.text.draw("Music Playing - Press 'P' to Pause", 0.0, 0.0);
        } else {
            ctx.text.draw("Music Paused - Press 'P' to Resume", 0.0, 0.0);
        }
    }
}

fn main() {
    run(AppConfig::default(), MyGame::default());
}
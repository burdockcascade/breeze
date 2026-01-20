use breeze::prelude::*;

// ============================================================================
// 1. THE TITLE SCREEN
// ============================================================================
struct TitleScreen;

impl Scene for TitleScreen {
    fn update(&mut self, ctx: &mut Context) -> SceneTransition {
        if ctx.input.key_pressed(KeyCode::Enter) {
            return SceneTransition::Switch(Box::new(GameScreen::default()));
        }
        SceneTransition::None
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        ctx.clear_background(Color::WHITE);

        // UI LAYER (Layer 1)
        ctx.with_layer(1, |ui| {
            ui.set_camera(CameraMode::default());
            ui.text.draw("BREEZE ENGINE", Vec2::new(0.0, 50.0));
            ui.text.draw("Press ENTER to Start", Vec2::new(0.0, -50.0));
        });
    }
}

// ============================================================================
// 2. THE GAMEPLAY SCREEN
// ============================================================================
#[derive(Default)]
struct GameScreen {
    player_pos: Vec2,
    player_sprite: ImageAsset,
}

impl Scene for GameScreen {
    fn init(&mut self, ctx: &mut Context) {
        self.player_sprite = ctx.load_image("breeze.png");
    }

    fn update(&mut self, ctx: &mut Context) -> SceneTransition {
        if ctx.input.key_down(KeyCode::ArrowRight) { self.player_pos.x += 5.0; }
        if ctx.input.key_down(KeyCode::ArrowLeft)  { self.player_pos.x -= 5.0; }

        if ctx.input.key_pressed(KeyCode::Space) {
            return SceneTransition::Push(Box::new(PauseScreen));
        }

        SceneTransition::None
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        ctx.clear_background(Color::srgb(0.1, 0.1, 0.2));

        // WORLD LAYER (Layer 0)
        ctx.with_layer(0, |world| {
            // Setup camera for the world if needed (e.g. follow player)
            world.set_camera(CameraMode::default());

            world.sprites.draw(&self.player_sprite, self.player_pos.x, self.player_pos.y);
        });

        // UI LAYER (Layer 1) - Drawn on top of layer 0
        ctx.with_layer(1, |ui| {
            ui.set_camera(CameraMode::default());
            ui.text.draw("Press SPACE to Pause", Vec2::new(0.0, -200.0));
        });
    }
}

// ============================================================================
// 3. THE PAUSE OVERLAY
// ============================================================================
struct PauseScreen;

impl Scene for PauseScreen {
    fn update(&mut self, ctx: &mut Context) -> SceneTransition {
        if ctx.input.key_pressed(KeyCode::Space) {
            return SceneTransition::Pop;
        }
        SceneTransition::None
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        // We do NOT clear background so we see the game behind us

        // OVERLAY LAYER (Layer 2) - Higher than Game(0) and GameUI(1)
        ctx.with_layer(2, |overlay| {
            overlay.set_camera(CameraMode::default());

            // Semi-transparent black box
            overlay.draw2d.rect(Vec2::ZERO, Vec2::new(1280.0, 720.0), Color::srgba(0.0, 0.0, 0.0, 0.7));
            overlay.text.draw("PAUSED", Vec2::ZERO);
        });
    }
}

fn main() {
    Breeze::new()
        .title("Scene Management with Layers")
        .resolution(1280, 720)
        .run(TitleScreen);
}
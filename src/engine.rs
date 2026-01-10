use bevy::ecs::system::SystemParam;
use bevy::prelude::*;

pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            width: 800,
            height: 600,
        }
    }
}

pub struct Context<'a> {
    pub time: &'a Time,
}

pub struct DrawContext<'a> {
    pub time: &'a Time,
}

pub trait Game: Send + Sync + 'static {
    fn init(&mut self, _ctx: &mut Context) {}
    fn update(&mut self, ctx: &mut Context);
    fn draw(&mut self, ctx: &mut DrawContext);
}

#[derive(Default)]
pub struct InternalState { initialized: bool }

#[derive(SystemParam)]
pub struct EngineContext<'w> {

    // Core
    pub time: Res<'w, Time>

}

pub fn internal_game_loop<G: Game>(mut game: NonSendMut<G>, engine: EngineContext, mut state: Local<InternalState>) {

    // --- UPDATE STEP ---
    {
        let mut ctx = Context {
            time: &engine.time,
        };

        if !state.initialized {
            game.init(&mut ctx);
            state.initialized = true;
        }

        game.update(&mut ctx);
    }

    // --- DRAW STEP ---
    {
        let mut draw_ctx = DrawContext {
            time: &engine.time,
        };
        game.draw(&mut draw_ctx);
    }
}
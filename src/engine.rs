use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy_vector_shapes::painter::ShapePainter;
use crate::shapes::ShapeContext;
use crate::text::{TextContext, TextQueue};

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

pub struct DrawContext<'a, 'w, 's> {
    pub time: &'a Time,
    pub shapes: ShapeContext<'a, 'w, 's>,
    pub text: TextContext<'a, 'w>,
}

pub trait Game: Send + Sync + 'static {
    fn init(&mut self, _ctx: &mut Context) {}
    fn update(&mut self, ctx: &mut Context);
    fn draw(&mut self, ctx: &mut DrawContext);
}

#[derive(Default)]
pub struct InternalState { initialized: bool }

#[derive(SystemParam)]
pub struct EngineContext<'w, 's> {

    // Core
    pub time: Res<'w, Time>,

    // Graphics
    pub painter: ShapePainter<'w, 's>,

    // Queues
    pub text_queue: ResMut<'w, TextQueue>,

}

pub fn internal_game_loop<G: Game>(mut game: NonSendMut<G>, mut engine: EngineContext, mut state: Local<InternalState>) {

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
            shapes: ShapeContext::new(&mut engine.painter),
            text: TextContext::new(&mut engine.text_queue),
        };
        game.draw(&mut draw_ctx);
    }
}
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_vector_shapes::painter::ShapePainter;
use crate::audio::{AudioContext, AudioQueue};
use crate::input::InputContext;
use crate::shapes::ShapeContext;
use crate::sprite::{SpriteContext, SpriteQueue};
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
    pub input: InputContext<'a>,
    pub asset_server: &'a AssetServer,
    pub audio: AudioContext<'a>,
}

impl<'a> Context<'a> {
    /// Load an image from the "assets" folder
    pub fn load_image(&self, path: &str) -> Handle<Image> {
        self.asset_server.load(path.to_owned())
    }
}

pub struct DrawContext<'a, 'w, 's> {
    pub time: &'a Time,
    pub shapes: ShapeContext<'a, 'w, 's>,
    pub text: TextContext<'a, 'w>,
    pub sprites: SpriteContext<'a, 'w>,
    pub(crate) clear_color: &'a mut ClearColor,
}

impl <'a, 'w, 's> DrawContext<'a, 'w, 's> {
    /// Set the background clear color for the next frame
    pub fn clear_background(&mut self, color: Color) {
        self.clear_color.0 = color;
    }
}

pub trait Game: Send + Sync + 'static {
    fn init(&mut self, _ctx: &mut Context) {}
    fn update(&mut self, _ctx: &mut Context) {}
    fn draw(&mut self, ctx: &mut DrawContext);
}

#[derive(Default)]
pub struct InternalState { initialized: bool }

#[derive(SystemParam)]
pub struct EngineContext<'w, 's> {

    // Core
    pub time: Res<'w, Time>,
    pub asset_server: Res<'w, AssetServer>,

    // Graphics
    pub painter: ShapePainter<'w, 's>,

    // Queues
    pub text_queue: ResMut<'w, TextQueue>,
    pub sprite_queue: ResMut<'w, SpriteQueue>,
    pub audio_queue: ResMut<'w, AudioQueue>,

    // Input
    pub keys: Res<'w, ButtonInput<KeyCode>>,
    pub mouse_buttons: Res<'w, ButtonInput<MouseButton>>,

    // Window / Camera (for mouse calculation)
    pub q_window: Query<'w, 's, &'static Window, With<PrimaryWindow>>,
    pub q_camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<Camera2d>>,

    // Clear Color
    pub clear_color: ResMut<'w, ClearColor>,

}

pub fn internal_game_loop<G: Game>(mut game: NonSendMut<G>, mut engine: EngineContext, mut state: Local<InternalState>) {

    let mut cursor_world_pos = Vec2::ZERO;
    if let (Ok(window), Ok((camera, camera_transform))) = (engine.q_window.single(), engine.q_camera.single()) {
        if let Some(screen_pos) = window.cursor_position() {
            // Convert Screen (Top-Left) -> World (Center)
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
                cursor_world_pos = world_pos;
            }
        }
    }

    // --- UPDATE STEP ---
    {
        let mut ctx = Context {
            time: &engine.time,
            input: InputContext {
                keys: &engine.keys,
                mouse_buttons: &engine.mouse_buttons,
                cursor_world_pos,
            },
            asset_server: &engine.asset_server,
            audio: AudioContext {
                queue: &mut engine.audio_queue,
                asset_server: &engine.asset_server,
            },
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
            sprites: SpriteContext::new(&mut engine.sprite_queue, &engine.asset_server),
            clear_color: &mut engine.clear_color,
        };
        game.draw(&mut draw_ctx);
    }
}
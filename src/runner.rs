use bevy::camera::visibility::RenderLayers;
use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::system::SystemParam;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::{PresentMode, PrimaryWindow};

use crate::core::audio::{play_audio, ActiveLoops, AudioContext, AudioQueue};
use crate::camera::{manage_cameras, CameraQueue};
use crate::context::{Context, DrawContext};
use crate::core::fps::{monitor_fps, FpsResource};
use crate::core::input::InputContext;
use crate::core::window::WindowContext;

use crate::graphics::commands::GraphicsQueue;
use crate::graphics::renderer::render_graphics;
use crate::graphics::geometry::{GlobalGeometryResources, MaterialCache};

pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub enable_logging: bool,
    pub enable_diagnostics: bool,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            width: 800,
            height: 600,
            enable_logging: false,
            enable_diagnostics: false,
        }
    }
}

/// A builder for configuring and running a Breeze game.
pub struct Breeze {
    config: AppConfig,
}

impl Breeze {
    /// Create a new Breeze builder with default settings.
    pub fn new() -> Self {
        Self {
            config: AppConfig::default(),
        }
    }

    /// Set the window title.
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }

    /// Set the window resolution (width and height).
    pub fn resolution(mut self, width: u32, height: u32) -> Self {
        self.config.width = width;
        self.config.height = height;
        self
    }

    /// Enable or disable logging.
    pub fn enable_logging(mut self, enable: bool) -> Self {
        self.config.enable_logging = enable;
        self
    }

    /// Enable or disable diagnostics.
    pub fn enable_diagnostics(mut self, enable: bool) -> Self {
        self.config.enable_diagnostics = enable;
        self
    }

    /// Consumes the builder and runs the game.
    pub fn run<G: Game>(self, game: G) {
        run(self.config, game);
    }
}

impl Default for Breeze {
    fn default() -> Self {
        Self {
            config: AppConfig::default(),
        }
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
    pub time: Res<'w, Time>,
    pub asset_server: Res<'w, AssetServer>,

    pub fps: Res<'w, FpsResource>,

    // Queues
    pub camera_queue: ResMut<'w, CameraQueue>,
    pub audio_queue: ResMut<'w, AudioQueue>,
    pub graphics_queue: ResMut<'w, GraphicsQueue>,

    pub keys: Res<'w, ButtonInput<KeyCode>>,
    pub mouse_buttons: Res<'w, ButtonInput<MouseButton>>,

    pub q_window: Query<'w, 's, &'static mut Window, With<PrimaryWindow>>,
    pub q_camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform, Option<&'static RenderLayers>), With<Camera>>,

    pub clear_color: ResMut<'w, ClearColor>,
}

pub fn internal_game_loop<G: Game>(mut game: NonSendMut<G>, mut engine: EngineContext, mut state: Local<InternalState>) {

    let mut primary_window_result = engine.q_window.single_mut();

    if let Ok(ref mut window) = primary_window_result {

        let mut cursor_world_pos = Vec2::ZERO;
        let target_layer_id = 0;

        // (Input handling code remains same...)
        let input_camera = engine.q_camera.iter().find(|(_, _, layers)| {
            match layers {
                Some(l) => l.intersects(&RenderLayers::layer(target_layer_id)),
                None => true,
            }
        });

        if let Some((camera, camera_transform, _)) = input_camera {
            if let Some(screen_pos) = window.cursor_position() {
                if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
                    cursor_world_pos = world_pos;
                }
            }
        }

        {
            let mut ctx = Context {
                time: &engine.time,
                fps: &engine.fps,
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
                window: WindowContext {
                    window
                },
            };

            if !state.initialized {
                game.init(&mut ctx);
                state.initialized = true;
            }

            game.update(&mut ctx);
        }

        {
            let mut draw_ctx = DrawContext {
                time: &engine.time,
                fps: &engine.fps,
                graphics_queue: &mut engine.graphics_queue,
                asset_server: &engine.asset_server,
                clear_color: &mut engine.clear_color,
                camera_queue: &mut engine.camera_queue,
            };
            game.draw(&mut draw_ctx);
        }
    }
}

fn run<G: Game>(config: AppConfig, game: G) {
    let mut binding = App::new();

    let default_plugin_set = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: config.title,
            resolution: (config.width, config.height).into(),
            present_mode: PresentMode::AutoVsync,
            ..default()
        }),
        ..default()
    });

    let app = binding
        .add_plugins(default_plugin_set)
        .init_resource::<GlobalGeometryResources>()
        .init_resource::<MaterialCache>()
        .init_resource::<FpsResource>()
        .insert_resource(GraphicsQueue::default()) // The One Queue
        .insert_resource(AudioQueue::default())
        .insert_resource(ActiveLoops::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(CameraQueue::default())
        .insert_non_send_resource(game)
        .add_systems(Update, (
            internal_game_loop::<G>,
            monitor_fps,
            render_graphics,
            play_audio,
            manage_cameras
        ).chain());

    if config.enable_diagnostics {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
            EntityCountDiagnosticsPlugin::default(),
        ));
    }

    if config.enable_logging {
        app.add_plugins(LogPlugin::default());
    }

    app.run();
}
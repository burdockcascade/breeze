use crate::core::scene::Scene;
use bevy::camera::visibility::RenderLayers;
use bevy::diagnostic::{EntityCountDiagnosticsPlugin, FrameCount, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::ecs::system::SystemParam;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::renderer::RenderAdapterInfo;
use bevy::window::{PresentMode, PrimaryWindow};
use bevy::winit::WinitWindows;
use crate::core::audio::{play_audio, ActiveLoops, AudioContext, AudioQueue};
use crate::camera::{manage_cameras, CameraQueue};
use crate::context::{Context, DrawContext};
use crate::core::fps::{monitor_fps, FpsResource};
use crate::core::input::InputContext;
use crate::core::scene::SceneManager;
use crate::core::system::{MonitorInfo, SystemContext};
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
    pub fn run(self, initial_scene: impl Scene + 'static) {
        let manager = SceneManager::new(initial_scene);
        run(self.config, manager);
    }
}

impl Default for Breeze {
    fn default() -> Self {
        Self {
            config: AppConfig::default(),
        }
    }
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

    pub renderer_info: Option<Res<'w, RenderAdapterInfo>>,
    pub frame_count: Option<Res<'w, FrameCount>>,

    pub q_window: Query<'w, 's, &'static mut Window, With<PrimaryWindow>>,
    pub q_camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform, Option<&'static RenderLayers>), With<Camera>>,

    pub clear_color: ResMut<'w, ClearColor>,
}

pub fn internal_game_loop(mut manager: NonSendMut<SceneManager>, mut engine: EngineContext, mut state: Local<InternalState>, winit_windows: Option<NonSend<WinitWindows>>,) {

    if manager.should_quit {
        return;
    }

    let mut monitor_list = Vec::new();

    if let Some(winit) = &winit_windows {
        // Iterate over all actual windows (usually just one, but we check the monitors it sees)
        for window in winit.windows.values() {
            if let Some(monitor) = window.current_monitor() {
                // In a real multi-monitor setup, you might iterate winit's `available_monitors()`
                // But accessing the current one is the safest default for now.

                let size = monitor.size();
                // Refresh rate is often wrapped in video modes,
                // defaulting to 60.0 if we can't easily query modes here.
                let refresh_rate = monitor.refresh_rate_millihertz().unwrap_or(60000) as f32 / 1000.0;

                monitor_list.push(MonitorInfo {
                    name: monitor.name().unwrap_or_else(|| "Generic Display".into()),
                    width: size.width,
                    height: size.height,
                    refresh_rate,
                    scale_factor: monitor.scale_factor(),
                });
            }
        }
    }

    // Fallback if list is empty (e.g. headless or first frame weirdness)
    if monitor_list.is_empty() {
        monitor_list.push(MonitorInfo {
            name: "Unknown".into(),
            width: 0,
            height: 0,
            refresh_rate: 0.0,
            scale_factor: 1.0,
        });
    }

    // Extract GPU info if available
    let (gpu_name, backend) = if let Some(info) = engine.renderer_info {
        (Some(info.name.clone()), Some(format!("{:?}", info.backend)))
    } else {
        (None, None)
    };

    // Extract Frame Count
    let frame_count = engine.frame_count.map(|f| f.0).unwrap_or(0);

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
                system: SystemContext {
                    gpu_name,
                    backend,
                    frame_count,
                    monitors: monitor_list,
                },
            };

            if !state.initialized {
                if let Some(scene) = manager.stack.last_mut() {
                    scene.init(&mut ctx);
                }
                state.initialized = true;
            }

            manager.update(&mut ctx);
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
            manager.draw(&mut draw_ctx);
        }
    }
}

fn run(config: AppConfig, manager: SceneManager) {
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
        .insert_non_send_resource(manager)
        .add_systems(Update, (
            internal_game_loop,
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

    app.run();
}
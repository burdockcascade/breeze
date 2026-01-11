use bevy::camera::visibility::RenderLayers;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_vector_shapes::painter::ShapePainter;
use bevy_vector_shapes::Shape2dPlugin;
use crate::core::audio::{play_audio, ActiveLoops, AudioContext, AudioQueue};
use crate::camera::{manage_cameras, CameraQueue};
use crate::context::{AppConfig, Context, DrawContext};
use crate::core::input::InputContext;
use crate::graphics::sprite::{render_sprites, SpriteQueue};
use crate::graphics::text::{render_text, TextQueue};
use crate::core::window::WindowContext;

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
    pub camera_queue: ResMut<'w, CameraQueue>,
    pub text_queue: ResMut<'w, TextQueue>,
    pub sprite_queue: ResMut<'w, SpriteQueue>,
    pub audio_queue: ResMut<'w, AudioQueue>,

    // Input
    pub keys: Res<'w, ButtonInput<KeyCode>>,
    pub mouse_buttons: Res<'w, ButtonInput<MouseButton>>,

    // Window / Camera (for mouse calculation)
    pub q_window: Query<'w, 's, &'static mut Window, With<PrimaryWindow>>,
    pub q_camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform, Option<&'static RenderLayers>), With<Camera>>,

    // Clear Color
    pub clear_color: ResMut<'w, ClearColor>,

}

pub fn internal_game_loop<G: Game>(mut game: NonSendMut<G>, mut engine: EngineContext, mut state: Local<InternalState>) {

    let mut primary_window_result = engine.q_window.single_mut();

    if let Ok(ref mut window) = primary_window_result {

        let mut cursor_world_pos = Vec2::ZERO;

        let target_layer_id = 0;

        // Find the camera that renders this specific layer
        let input_camera = engine.q_camera.iter().find(|(_, _, layers)| {
            match layers {
                Some(l) => l.intersects(&RenderLayers::layer(target_layer_id)),
                None => true,
            }
        });

        // Calculate cursor world position
        if let Some((camera, camera_transform, _)) = input_camera {
            if let Some(screen_pos) = window.cursor_position() {
                if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, screen_pos) {
                    cursor_world_pos = world_pos;
                }
            }
        }

        // Call Update
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

        // Call Draw
        {
            let mut draw_ctx = DrawContext {
                time: &engine.time,
                painter: &mut engine.painter,
                sprite_queue: &mut engine.sprite_queue,
                text_queue: &mut engine.text_queue,
                asset_server: &engine.asset_server,
                clear_color: &mut engine.clear_color,
                camera_queue: &mut engine.camera_queue,
            };
            game.draw(&mut draw_ctx);
        }

    }
}

pub fn run<G: Game>(config: AppConfig, game: G) {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: config.title,
                    resolution: (config.width, config.height).into(),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(Shape2dPlugin::default())
        .insert_resource(TextQueue::default())
        .insert_resource(SpriteQueue::default())
        .insert_resource(AudioQueue::default())
        .insert_resource(ActiveLoops::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(CameraQueue::default())
        .insert_non_send_resource(game)
        .add_systems(Update, (
            internal_game_loop::<G>,
            render_text,
            render_sprites,
            play_audio,
            manage_cameras
        ).chain())
        .run();
}
mod engine;
mod shapes;
mod text;
mod input;
mod sprite;

use bevy::prelude::*;
use bevy_vector_shapes::prelude::*;
use crate::engine::*;
use crate::sprite::{render_sprites, SpriteQueue};
use crate::text::{render_text, TextQueue};

pub mod prelude {
    pub use crate::run;
    pub use crate::engine::*;
    pub use bevy::prelude::*;
    pub use bevy::color::palettes::css::*;
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn run<G: Game>(config: AppConfig, game: G) {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: config.title,
                resolution: (config.width, config.height).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(Shape2dPlugin::default())
        .insert_resource(TextQueue::default())
        .insert_resource(SpriteQueue::default())
        .insert_non_send_resource(game)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, (
            internal_game_loop::<G>,
            render_text,
            render_sprites,
        ).chain())
        .run();
}
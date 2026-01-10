mod engine;

use bevy::prelude::*;
use crate::engine::*;

pub mod prelude {
    pub use crate::run;
    pub use crate::engine::*;
    pub use bevy::prelude::*;
    pub use bevy::color::palettes::css::*;
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
        .insert_non_send_resource(game)
        .add_systems(Update, (
            internal_game_loop::<G>,
        ).chain())
        .run();
}
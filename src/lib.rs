mod engine;
mod shapes;
mod text;
mod input;
mod sprite;
mod audio;
mod window;

pub mod prelude {
    pub use crate::engine::*;
    pub use bevy::prelude::*;
    pub use bevy::color::palettes::css::*;
}
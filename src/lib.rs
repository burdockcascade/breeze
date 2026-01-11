mod engine;
mod shapes;
mod text;
mod input;
mod sprite;
mod audio;
mod window;
mod camera;

pub mod prelude {
    pub use crate::engine::*;
    pub use crate::camera::*;
    pub use bevy::prelude::*;
    pub use bevy::color::palettes::css::*;
}
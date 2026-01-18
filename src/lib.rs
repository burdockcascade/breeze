mod runner;
mod camera;
mod context;
mod core;
mod graphics;

pub mod prelude {
    pub use crate::runner::{Game, Breeze};
    pub use crate::context::{Context, DrawContext, LayerContext};
    pub use crate::camera::CameraMode;
    pub use bevy::prelude::*;
    pub use bevy::color::palettes::css::*;
}
mod runner;
mod camera;
mod context;
mod common;
mod core;
mod graphics;

pub mod prelude {
    pub use crate::runner::{run, Game};
    pub use crate::context::{AppConfig, Context, DrawContext};
    pub use crate::camera::CameraMode;
    pub use bevy::prelude::*;
    pub use bevy::color::palettes::css::*;
}
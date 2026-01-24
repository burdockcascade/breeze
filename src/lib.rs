use bevy::prelude::{Handle, Image, Scene};

mod runner;
mod camera;
mod context;
mod core;
mod graphics;

/// Type aliases for common asset handles
pub type SceneAsset = Handle<Scene>;

/// Type aliases for common asset handles
pub type ImageAsset = Handle<Image>;

/// Type aliases for common asset handles
pub type FontAsset  = Handle<bevy::prelude::Font>;

/// Type aliases for common asset handles
pub type SoundAsset = Handle<bevy::prelude::AudioSource>;

/// Type aliases for common asset handles
pub type TextureAsset = Handle<Image>;

pub mod prelude {
    pub use crate::core::scene::{Scene, SceneTransition, SceneManager};

    // Export Breeze types
    pub use crate::{SceneAsset, ImageAsset, FontAsset, SoundAsset};

    pub use crate::runner::{Breeze};
    pub use crate::context::{Context, DrawContext, LayerContext};
    pub use crate::camera::CameraMode;
    pub use bevy::color::palettes::css::*;

    // basic bevy types
    pub use bevy::prelude::{vec2, vec3, vec4, Vec2, Vec3, Vec4, Quat, Color, KeyCode, MouseButton};
}
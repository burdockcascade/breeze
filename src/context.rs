use bevy::prelude::*;
use crate::core::audio::AudioContext;
use crate::camera::{CameraMode, CameraQueue};
use crate::core::input::InputContext;
use crate::graphics::shapes::*;
use crate::graphics::sprite::{SpriteContext, SpriteQueue};
use crate::graphics::text::{TextContext, TextQueue};
use crate::core::window::WindowContext;

pub struct AppConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            title: "Untitled".to_string(),
            width: 800,
            height: 600,
        }
    }
}

pub struct Context<'a> {
    pub time: &'a Time,
    pub input: InputContext<'a>,
    pub asset_server: &'a AssetServer,
    pub audio: AudioContext<'a>,
    pub window: WindowContext<'a>,
}

impl<'a> Context<'a> {
    /// Load an image from the "assets" folder
    pub fn load_image(&self, path: &str) -> Handle<Image> {
        self.asset_server.load(path.to_owned())
    }
}

pub struct LayerContext<'a> {
    pub layer_id: usize,
    pub sprites: SpriteContext<'a>,
    pub text: TextContext<'a>,
pub shapes: ShapeContext<'a>,
    pub camera_queue: &'a mut CameraQueue,
}

impl<'a> LayerContext<'a> {
    // New API: Set the camera mode for this specific layer
    pub fn set_camera(&mut self, mode: CameraMode) {
        self.camera_queue.0.push((self.layer_id, mode));
    }
}

pub struct DrawContext<'a> {
    pub time: &'a Time,
    pub shape_queue: &'a mut ShapeQueue,
    pub sprite_queue: &'a mut SpriteQueue,
    pub text_queue: &'a mut TextQueue,
    pub asset_server: &'a AssetServer,
    pub camera_queue: &'a mut CameraQueue,
    pub clear_color: &'a mut ClearColor,
}

impl <'a> DrawContext<'a> {

    pub fn with_layer<F>(&mut self, id: usize, f: F)
    where
        F: FnOnce(&mut LayerContext)
    {
        let mut ctx = LayerContext {
            layer_id: id,
            sprites: SpriteContext {
                queue: self.sprite_queue, // Re-borrow the queue
                asset_server: self.asset_server,
                layer_id: id, // Set the ID
            },
            text: TextContext {
                queue: self.text_queue,
                layer_id: id,
            },
            shapes: ShapeContext::new(self.shape_queue, id),
            camera_queue: self.camera_queue,
        };

        f(&mut ctx);
    }

    /// Set the background clear color for the next frame
    pub fn clear_background(&mut self, color: Color) {
        self.clear_color.0 = color;
    }
}
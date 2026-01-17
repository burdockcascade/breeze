use bevy::prelude::*;
use std::cell::RefCell;

use crate::core::audio::AudioContext;
use crate::camera::{CameraMode, CameraQueue};
use crate::core::input::InputContext;
use crate::core::window::WindowContext;

// --- NEW UNIFIED IMPORTS ---
use crate::graphics::commands::GraphicsQueue;
use crate::graphics::geometry::{Geometry2d, Geometry3d};
use crate::graphics::lights::LightContext;
use crate::graphics::sprite::SpriteContext;
use crate::graphics::text::TextContext;
// ---------------------------

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
    pub fn load_image(&self, path: &str) -> Handle<Image> {
        self.asset_server.load(path.to_owned())
    }

    pub fn load_scene(&self, path: &str) -> Handle<Scene> {
        self.asset_server.load(path.to_owned())
    }

    pub fn load_font(&self, path: &str) -> Handle<Font> {
        self.asset_server.load(path.to_owned())
    }
}

pub struct LayerContext<'a> {
    pub layer_id: usize,

    // The Single Source of Truth
    pub queue: &'a RefCell<&'a mut GraphicsQueue>,

    // Helper wrappers
    pub draw2d: Geometry2d<'a>,
    pub draw3d: Geometry3d<'a>,
    pub sprites: SpriteContext<'a>,
    pub text: TextContext<'a>,
    pub lights: LightContext<'a>,

    pub camera_queue: &'a mut CameraQueue,
}

impl<'a> LayerContext<'a> {
    pub fn set_camera(&mut self, mode: CameraMode) {
        self.camera_queue.0.push((self.layer_id, mode));
    }
}

pub struct DrawContext<'a> {
    pub time: &'a Time,

    // --- SINGLE QUEUE ---
    pub graphics_queue: &'a mut GraphicsQueue,
    // --------------------

    pub asset_server: &'a AssetServer,
    pub camera_queue: &'a mut CameraQueue,
    pub clear_color: &'a mut ClearColor,
}

impl <'a> DrawContext<'a> {

    pub fn with_layer<F>(&mut self, id: usize, f: F)
    where
        F: FnOnce(&mut LayerContext)
    {
        // 1. Wrap the mutable queue in a RefCell
        // This allows all sub-contexts to "share" the mutable reference safely
        let queue_cell = RefCell::new(&mut *self.graphics_queue);

        let mut ctx = LayerContext {
            layer_id: id,
            queue: &queue_cell, // Store it if needed directly

            // 2. Initialize all sub-contexts with the SAME cell
            draw2d: Geometry2d { queue: &queue_cell, layer_id: id },
            draw3d: Geometry3d { queue: &queue_cell, layer_id: id },

            sprites: SpriteContext {
                queue: &queue_cell,
                asset_server: self.asset_server,
                layer_id: id
            },

            text: TextContext {
                queue: &queue_cell,
                layer_id: id
            },

            lights: LightContext {
                queue: &queue_cell,
                layer_id: id
            },

            camera_queue: self.camera_queue,
        };

        f(&mut ctx);
    }

    pub fn clear_background(&mut self, color: Color) {
        self.clear_color.0 = color;
    }
}
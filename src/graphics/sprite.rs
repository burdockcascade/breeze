use bevy::prelude::*;
use bevy::camera::visibility::RenderLayers;
use bevy::ecs::system::SystemParam;
use std::cell::RefCell;

// Import the Unified Types (Assuming you created src/graphics/commands.rs)
use crate::graphics::commands::{GraphicsCommand, GraphicsQueue};

// --- 1. COMMAND DATA ---
#[derive(Clone)]
pub struct SpriteCommand {
    pub image: Handle<Image>,
    pub position: Vec2,
    pub scale: Vec2,
    pub color: Color,
    pub layer: usize,
}

// --- 2. MARKER COMPONENT (For Cleanup) ---
#[derive(Component)]
pub struct ImmediateSprite;

// --- 3. CONTEXT (Frontend) ---
pub struct SpriteContext<'a> {
    // Reference to the Global Queue via RefCell
    pub queue: &'a RefCell<&'a mut GraphicsQueue>,
    pub asset_server: &'a AssetServer,
    pub layer_id: usize,
}

impl<'a> SpriteContext<'a> {

    pub fn load(&self, path: &str) -> Handle<Image> {
        self.asset_server.load(path.to_owned())
    }

    pub fn draw(&self, image: &Handle<Image>, x: f32, y: f32) {
        self.draw_ext(image, x, y, 1.0, Color::WHITE);
    }

    pub fn draw_ext(&self, image: &Handle<Image>, x: f32, y: f32, scale: f32, color: Color) {
        // Push to the Global Queue
        self.queue.borrow_mut().0.push(GraphicsCommand::Sprite(SpriteCommand {
            image: image.clone(),
            position: Vec2::new(x, y),
            scale: Vec2::splat(scale),
            color,
            layer: self.layer_id,
        }));
    }
}

// --- 4. RENDERER RESOURCES (Backend) ---
#[derive(SystemParam)]
pub struct SpriteRenderer<'w, 's> {
    // Query to find sprites from the previous frame to clean them up
    pub q_sprites: Query<'w, 's, Entity, With<ImmediateSprite>>,
}

// --- 5. SPAWN HELPER (Called by UnifiedRenderer) ---
pub fn process_sprite(
    commands: &mut Commands,
    entity_opt: Option<Entity>,
    cmd: SpriteCommand
) {
    let mut e = if let Some(entity) = entity_opt {
        commands.entity(entity)
    } else {
        commands.spawn((
            ImmediateSprite, // Only needed on creation
        ))
    };

    e.insert((
        Sprite {
            image: cmd.image,
            color: cmd.color,
            ..default()
        },
        Transform::from_translation(cmd.position.extend(0.0))
            .with_scale(cmd.scale.extend(1.0)),
        RenderLayers::layer(cmd.layer),
        Visibility::Visible, // Un-hide if recycled
    ));
}
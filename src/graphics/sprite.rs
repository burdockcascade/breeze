use bevy::prelude::*;
use bevy::camera::visibility::RenderLayers;
use bevy::ecs::system::SystemParam;
use std::cell::RefCell;
use crate::graphics::commands::{GraphicsCommand, GraphicsQueue};

// ... SpriteCommand, ImmediateSprite, SpriteContext (Same as before) ...
#[derive(Clone)]
pub struct SpriteCommand {
    pub image: Handle<Image>,
    pub position: Vec2,
    pub scale: Vec2,
    pub color: Color,
    pub layer: usize,
}

#[derive(Component)]
pub struct ImmediateSprite;

pub struct SpriteContext<'a> {
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
    pub q_sprites: Query<'w, 's, (
        Entity,
        &'static mut Sprite,
        &'static mut Transform,
        &'static mut Visibility,
        &'static mut RenderLayers
    ), With<ImmediateSprite>>,

    pub _marker: std::marker::PhantomData<&'s ()>,
}

// --- OPTIMIZED PROCESS FUNCTION ---
pub fn process_sprite(
    commands: &mut Commands,
    renderer: &mut SpriteRenderer, // Now takes mutable renderer
    entity_opt: Option<Entity>,
    cmd: SpriteCommand
) {
    // 1. FAST PATH: Direct Mutation (Zero Allocation, Zero Command Overhead)
    if let Some(entity) = entity_opt {
        if let Ok((_, mut sprite, mut transform, mut vis, mut layers)) = renderer.q_sprites.get_mut(entity) {

            // Just assign the values! Bevy is very fast at this.
            sprite.image = cmd.image;
            sprite.color = cmd.color;
            // Reset standard fields in case they were changed (optional but safe)
            sprite.flip_x = false;
            sprite.flip_y = false;
            sprite.custom_size = None;

            transform.translation = cmd.position.extend(0.0);
            transform.scale = cmd.scale.extend(1.0);
            transform.rotation = Quat::IDENTITY; // Reset rotation

            *vis = Visibility::Visible; // Wake up hidden sprite
            *layers = RenderLayers::layer(cmd.layer);

            return; // Done! skipped commands.spawn entirely.
        }
    }

    // 2. SLOW PATH: Spawn New Entity (First run or pool empty)
    commands.spawn((
        Sprite {
            image: cmd.image,
            color: cmd.color,
            ..default()
        },
        Transform::from_translation(cmd.position.extend(0.0))
            .with_scale(cmd.scale.extend(1.0)),
        RenderLayers::layer(cmd.layer),
        Visibility::Visible,
        ImmediateSprite,
    ));
}
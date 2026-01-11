use bevy::camera::visibility::RenderLayers;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use crate::common::StableId;

// A command to draw a sprite
#[derive(Clone)]
pub struct SpriteCommand {
    pub image: Handle<Image>,
    pub position: Vec2,
    pub scale: Vec2,
    pub color: Color,
}

// A queue of sprite commands to be rendered each frame
#[derive(Resource, Default)]
pub struct SpriteQueue(pub Vec<Vec<SpriteCommand>>);

// Marker component for immediate mode sprites
#[derive(Component)]
pub struct ImmediateSprite;

pub struct SpriteContext<'a> {
    pub queue: &'a mut SpriteQueue,
    pub asset_server: &'a AssetServer,
    pub layer_id: usize,
}

impl<'a> SpriteContext<'a> {

    fn get_queue(&mut self) -> &mut Vec<SpriteCommand> {
        if self.layer_id >= self.queue.0.len() {
            self.queue.0.resize_with(self.layer_id + 1, Vec::new);
        }
        &mut self.queue.0[self.layer_id]
    }

    /// Draw a sprite at (x, y) with default scale and color
    pub fn draw(&mut self, image: &Handle<Image>, x: f32, y: f32) {
        self.draw_ext(image, x, y, 1.0, Color::WHITE);
    }

    /// Draw a scaled or tinted sprite
    pub fn draw_ext(&mut self, image: &Handle<Image>, x: f32, y: f32, scale: f32, color: Color) {
        self.get_queue().push(SpriteCommand {
            image: image.clone(),
            position: Vec2::new(x, y),
            scale: Vec2::splat(scale),
            color,
        });
    }

}

// Query data for sprite items
#[derive(QueryData)]
#[query_data(mutable)]
pub struct SpriteItem {
    pub entity: Entity,
    pub id: Option<&'static StableId>,
    pub transform: &'static mut Transform,
    pub sprite: &'static mut Sprite,
    pub visibility: &'static mut Visibility,
    pub layers: Option<&'static mut RenderLayers>,
}

// System to render sprites from the sprite queue
pub fn render_sprites( mut commands: Commands, mut queue: ResMut<SpriteQueue>, mut query: Query<SpriteItem, With<ImmediateSprite>>, mut flat_commands: Local<Vec<(usize, usize, SpriteCommand)>>) {

    // Flatten commands (same as before)
    flat_commands.clear();
    for (layer_id, cmds) in queue.0.iter().enumerate() {
        for (i, cmd) in cmds.iter().enumerate() {
            flat_commands.push((layer_id, i, cmd.clone()));
        }
    }

    // Build a Lookup Table (The "Stable" Map)
    let mut entity_lookup = Vec::new();

    // Resize to fit the largest ID we might encounter (heuristic)
    let max_capacity = flat_commands.len().max(query.iter().len());
    entity_lookup.resize_with(max_capacity, || None);

    // Populate the lookup
    for item in query.iter_mut() {
        if let Some(stable_id) = item.id {
            if stable_id.0 < entity_lookup.len() {
                entity_lookup[stable_id.0] = Some(item);
            } else {
                commands.entity(item.entity).despawn();
            }
        } else {
            // Entity missing an ID (shouldn't happen, but safe cleanup)
            commands.entity(item.entity).despawn();
        }
    }

    // Process Commands
    for (global_index, (layer_id, _sub_index, cmd)) in flat_commands.iter().enumerate() {

        let target_layer = RenderLayers::layer(*layer_id);

        // Does an entity exist for this draw call index?
        if let Some(Some(item)) = entity_lookup.get_mut(global_index) {

            // Target properties
            let z = (*layer_id as f32 * 100.0) + (global_index as f32 * 0.00001);
            let target_pos = cmd.position.extend(z);
            let target_scale = cmd.scale.extend(1.0);

            // Change Detection Optimization
            if item.transform.translation != target_pos {
                item.transform.translation = target_pos;
            }
            if item.transform.scale != target_scale {
                item.transform.scale = target_scale;
            }
            if item.sprite.image != cmd.image {
                item.sprite.image = cmd.image.clone();
            }
            if item.sprite.color != cmd.color {
                item.sprite.color = cmd.color;
            }
            if *item.visibility != Visibility::Visible {
                *item.visibility = Visibility::Visible;
            }

            // Layer Check
            if let Some(ref mut l) = item.layers {
                if **l != target_layer { **l = target_layer; }
            } else {
                commands.entity(item.entity).insert(target_layer);
            }

            // Remove from lookup so we don't double-process or despawn it later
            entity_lookup[global_index] = None;

        } else {
            commands.spawn((
                Sprite {
                    image: cmd.image.clone(),
                    color: cmd.color,
                    ..default()
                },
                Transform::from_translation(cmd.position.extend(0.0)),
                Visibility::Visible,
                ImmediateSprite,
                StableId(global_index),
                target_layer,
            ));
        }

    }

    // Hide or Despawn Unused Entities
    const MAX_RESERVE: usize = 100;
    let mut reserve_count = 0;

    for item_opt in entity_lookup.iter_mut() {
        if let Some(mut item) = item_opt.take() {
            if reserve_count < MAX_RESERVE {
                if *item.visibility != Visibility::Hidden {
                    *item.visibility = Visibility::Hidden;
                }
                reserve_count += 1;
            } else {
                commands.entity(item.entity).despawn();
            }
        }
    }

    // Cleanup
    for list in queue.0.iter_mut() {
        list.clear();
    }

}
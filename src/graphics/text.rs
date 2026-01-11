use bevy::camera::visibility::RenderLayers;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use crate::common::StableId;

// Command struct for queuing text rendering
#[derive(Clone)]
pub struct TextCommand {
    pub text: String,
    pub position: Vec2,
    pub size: f32,
    pub color: Color,
}

// Resource to hold queued text commands
#[derive(Resource, Default)]
pub struct TextQueue(pub Vec<Vec<TextCommand>>);

// Marker component for immediate text entities
#[derive(Component)]
pub struct ImmediateText;

pub struct TextContext<'a> {
    pub queue: &'a mut TextQueue,
    pub layer_id: usize,
}

impl<'a> TextContext<'a> {

    pub fn new(queue: &'a mut TextQueue, layer_id: usize) -> Self {
        Self {
            queue,
            layer_id,
        }
    }

    fn get_queue(&mut self) -> &mut Vec<TextCommand> {
        if self.layer_id >= self.queue.0.len() {
            self.queue.0.resize_with(self.layer_id + 1, Vec::new);
        }
        &mut self.queue.0[self.layer_id]
    }

    pub fn draw(&mut self, text: &str, x: f32, y: f32) {
       self.draw_ext(text, x, y, 16.0, Color::BLACK);
    }

    /// Draw text with size and color
    pub fn draw_ext(&mut self, text: &str, x: f32, y: f32, size: f32, color: Color) {
        self.get_queue().push(TextCommand {
            text: text.to_owned(),
            position: Vec2::new(x, y),
            size,
            color,
        });
    }

}

// QueryData struct for mutable access to text item components
#[derive(QueryData)]
#[query_data(mutable)]
pub struct TextItem {
    pub entity: Entity,
    pub id: Option<&'static StableId>,
    pub text: &'static mut Text2d,
    pub transform: &'static mut Transform,
    pub font: &'static mut TextFont,
    pub color: &'static mut TextColor,
    pub visibility: &'static mut Visibility,
    pub layers: Option<&'static mut RenderLayers>,
}

pub fn render_text(mut commands: Commands, mut queue: ResMut<TextQueue>, mut query: Query<TextItem, With<ImmediateText>>, mut flat_commands: Local<Vec<(usize, usize, TextCommand)>>) {

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

            // Update text only if changed
            if item.text.0 != cmd.text {
                item.text.0 = cmd.text.clone();
            }

            // Calculate z-index based on layer and index to ensure proper layering
            let z = (*layer_id as f32 * 100.0) + (global_index as f32 * 0.00001);

            // Update transform only if changed
            if item.transform.translation != Vec3::new(cmd.position.x, cmd.position.y, z) {
                item.transform.translation = Vec3::new(cmd.position.x, cmd.position.y, z);
            }

            // Update font size only if changed
            if item.font.font_size != cmd.size {
                item.font.font_size = cmd.size;
            }

            // Update color only if changed
            if item.color.0 != cmd.color {
                item.color.0 = cmd.color;
            }

            // Update visibility
            if *item.visibility != Visibility::Visible {
                *item.visibility = Visibility::Visible;
            }

            if let Some(ref mut l) = item.layers {
                if **l != target_layer { **l = target_layer; }
            } else {
                commands.entity(item.entity).insert(target_layer);
            }

            // Remove from lookup so we don't double-process or despawn it later
            entity_lookup[global_index] = None;

        } else {
            // Spawn new entity
            commands.spawn((
                Text2d::new(cmd.text.clone()),
                Transform::from_xyz(cmd.position.x, cmd.position.y, 1.0),
                TextFont { font_size: cmd.size, ..default() },
                TextColor(cmd.color),
                Visibility::Visible,
                ImmediateText,
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
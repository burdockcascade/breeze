use bevy::camera::visibility::RenderLayers;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;

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
    pub transform: &'static mut Transform,
    pub sprite: &'static mut Sprite,
    pub visibility: &'static mut  Visibility,
    pub layers: Option<&'static mut RenderLayers>,
}

// System to render sprites from the sprite queue
pub fn render_sprites( mut commands: Commands, mut queue: ResMut<SpriteQueue>, mut query: Query<SpriteItem, With<ImmediateSprite>>) {

    // Flatten
    let mut flat_commands = Vec::new();
    for (layer_id, cmds) in queue.0.iter().enumerate() {
        for (i, cmd) in cmds.iter().enumerate() {
            flat_commands.push((layer_id, i, cmd));
        }
    }

    let mut drawn_count = 0;

    // Recycle
    for (mut item, (layer_id, index, cmd)) in query.iter_mut().zip(flat_commands.iter()) {
        // Calculate Z based on layer to ensure proper sorting
        let z = (*layer_id as f32 * 100.0) + (*index as f32 * 0.0001);

        item.transform.translation = cmd.position.extend(z);
        item.transform.scale = cmd.scale.extend(1.0);
        item.sprite.image = cmd.image.clone();
        item.sprite.color = cmd.color;
        *item.visibility = Visibility::Visible;

        // Apply the correct RenderLayer
        let target_layer = RenderLayers::layer(*layer_id);
        if let Some(ref mut l) = item.layers {
            if **l != target_layer {
                **l = target_layer;
            }
        } else {
            commands.entity(item.entity).insert(target_layer);
        }

        drawn_count += 1;
    }

    // Spawn
    if flat_commands.len() > drawn_count {
        for (layer_id, _, cmd) in flat_commands.iter().skip(drawn_count) {
            commands.spawn((
                Sprite {
                    image: cmd.image.clone(),
                    color: cmd.color,
                    ..default()
                },
                Transform::from_translation(cmd.position.extend(0.0)),
                Visibility::Visible,
                ImmediateSprite,
                RenderLayers::layer(*layer_id),
            ));
        }
    }

    // Hide unused entities
    for mut item in query.iter_mut().skip(drawn_count) {
        *item.visibility = Visibility::Hidden;
    }

    // Cleanup
    for list in queue.0.iter_mut() {
        list.clear();
    }
}
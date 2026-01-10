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
pub struct SpriteQueue(pub(crate) Vec<SpriteCommand>);

// Marker component for immediate mode sprites
#[derive(Component)]
pub struct ImmediateSprite;

pub struct SpriteContext<'a, 'w> {
    pub queue: &'a mut ResMut<'w, SpriteQueue>,
    pub asset_server: &'a AssetServer,
}

impl<'a, 'w> SpriteContext<'a, 'w> {
    
    pub fn new(queue: &'a mut ResMut<'w, SpriteQueue>, asset_server: &'a AssetServer) -> Self {
        SpriteContext {
            queue,
            asset_server,
        }
    }
    
    pub fn sprite(&mut self, image: &Handle<Image>, x: f32, y: f32) {
        self.queue.0.push(SpriteCommand {
            image: image.clone(),
            position: Vec2::new(x, y),
            scale: Vec2::ONE,
            color: Color::WHITE,
        });
    }

    /// Draw a scaled or tinted sprite
    pub fn sprite_ext(&mut self, image: &Handle<Image>, x: f32, y: f32, scale: f32, color: Color) {
        self.queue.0.push(SpriteCommand {
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
}

// System to render sprites from the sprite queue
pub fn render_sprites( mut commands: Commands, mut queue: ResMut<SpriteQueue>, mut query: Query<SpriteItem, With<ImmediateSprite>>) {
    let mut drawn_count = 0;

    // We iterate through our pool of entities and the user's commands together
    for ((i, command), mut sprite_item) in queue.0.iter().enumerate().zip(query.iter_mut()) {

        // Update the entity to match the command
        let z_depth = i as f32 * 0.0001; // Slightly offset z to avoid z-fighting
        sprite_item.transform.translation = command.position.extend(z_depth);
        sprite_item.transform.scale = command.scale.extend(1.0);
        sprite_item.sprite.image = command.image.clone();
        sprite_item.sprite.color = command.color;

        // Make sure it's visible
        *sprite_item.visibility = Visibility::Visible;

        drawn_count += 1;
    }

    // If we have more commands than entities in the pool, spawn new ones
    if queue.0.len() > drawn_count {
        for command in queue.0.iter().skip(drawn_count) {
            commands.spawn((
                Sprite {
                    image: command.image.clone(),
                    color: command.color,
                    ..default()
                },
                Transform {
                    translation: command.position.extend(0.0),
                    scale: command.scale.extend(1.0),
                    ..default()
                },
                Visibility::Visible,
                ImmediateSprite,
            ));
        }
    }

    // Hide any remaining entities in the pool that were not used this frame
    for mut sprite_item in query.iter_mut().skip(drawn_count) {
        *sprite_item.visibility = Visibility::Hidden;
    }

    // Clear the queue for the next frame
    queue.0.clear();
}
use bevy::prelude::*;
use bevy::ecs::system::SystemParam;
use crate::graphics::commands::{GraphicsCommand, GraphicsQueue};
use crate::graphics::geometry::{GeometryRenderer, spawn_geometry};
use crate::graphics::sprite::{SpriteRenderer, spawn_sprite};
use crate::graphics::text::{TextRenderer, spawn_text};
use crate::graphics::lights::{LightRenderer, spawn_light};

#[derive(SystemParam)]
pub struct UnifiedRenderer<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub queue: ResMut<'w, GraphicsQueue>,
    pub geo_resources: GeometryRenderer<'w, 's>,
    pub sprite_resources: SpriteRenderer<'w, 's>,
    pub text_resources: TextRenderer<'w, 's>,
    pub light_resources: LightRenderer<'w, 's>,
}

pub fn render_graphics(mut renderer: UnifiedRenderer) {

    // --- 1. CLEANUP PHASE ---

    // Geometry
    for (entity, _) in renderer.geo_resources.q_transient.iter() {
        renderer.commands.entity(entity).despawn();
    }

    // Sprites
    for entity in renderer.sprite_resources.q_sprites.iter() {
        renderer.commands.entity(entity).despawn();
    }

    // Text (NEW)
    for entity in renderer.text_resources.q_text.iter() {
        renderer.commands.entity(entity).despawn();
    }

    // Lights (If not pooling)
    for entity in renderer.light_resources.q_lights.iter() {
        renderer.commands.entity(entity).despawn();
    }

    // --- 2. RENDERING PHASE ---
    for command in renderer.queue.0.drain(..) {
        match command {
            GraphicsCommand::Geometry(cmd) => {
                spawn_geometry(&mut renderer.commands, &mut renderer.geo_resources, cmd);
            },
            GraphicsCommand::Sprite(cmd) => {
                spawn_sprite(&mut renderer.commands, &renderer.sprite_resources, cmd);
            },
            GraphicsCommand::Text(cmd) => {
                spawn_text(&mut renderer.commands, &renderer.text_resources, cmd);
            },
            GraphicsCommand::Light(cmd) => {
                spawn_light(&mut renderer.commands, &renderer.light_resources, cmd);
            }
        }
    }
}
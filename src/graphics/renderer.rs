use bevy::prelude::*;
use bevy::ecs::system::SystemParam;

use crate::graphics::commands::{GraphicsCommand, GraphicsQueue};
use crate::graphics::geometry::{GeometryRenderer, process_geometry};
use crate::graphics::sprite::{SpriteRenderer, process_sprite};
use crate::graphics::text::{TextRenderer, process_text};
use crate::graphics::lights::{LightRenderer, process_light};

#[derive(SystemParam)]
pub struct UnifiedRenderer<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub queue: ResMut<'w, GraphicsQueue>,

    // Sub-renderers
    pub geo_resources: GeometryRenderer<'w, 's>,
    pub sprite_resources: SpriteRenderer<'w, 's>,
    pub text_resources: TextRenderer<'w, 's>,
    pub light_resources: LightRenderer<'w, 's>,
}

pub fn render_graphics(mut renderer: UnifiedRenderer) {
    // 1. COLLECT AVAILABLE ENTITIES (Pooling)
    // We pop entities from these vectors. If empty, we spawn new ones.

    // Geometry Pool
    // We filter for entities that have TransientResources (our geometry marker)
    let mut pool_geo: Vec<Entity> = renderer.geo_resources.q_transient.iter()
        .map(|(e, _)| e)
        .collect();

    // Sprite Pool
    let mut pool_sprites: Vec<Entity> = renderer.sprite_resources.q_sprites.iter()
        .collect();

    // Text Pool
    let mut pool_text: Vec<Entity> = renderer.text_resources.q_text.iter()
        .collect();

    // Light Pool
    let mut pool_lights: Vec<Entity> = renderer.light_resources.q_lights.iter()
        .collect();

    // 2. PROCESS COMMANDS
    for command in renderer.queue.0.drain(..) {
        match command {
            GraphicsCommand::Geometry(cmd) => {
                let entity = pool_geo.pop();
                process_geometry(&mut renderer.commands, &mut renderer.geo_resources, entity, cmd);
            },
            GraphicsCommand::Sprite(cmd) => {
                let entity = pool_sprites.pop();
                process_sprite(&mut renderer.commands, entity, cmd);
            },
            GraphicsCommand::Text(cmd) => {
                let entity = pool_text.pop();
                process_text(&mut renderer.commands, entity, cmd);
            },
            GraphicsCommand::Light(cmd) => {
                let entity = pool_lights.pop();
                process_light(&mut renderer.commands, entity, cmd);
            }
        }
    }

    // 3. HIDE UNUSED ENTITIES (Recycling)
    // Any entity left in the pools was not used this frame.
    // We hide them (soft recycle) or despawn them (hard recycle) if too many.

    // Geometry: MUST clean up assets even if just hiding, or just despawn excess.
    // For simplicity and to prevent material leaks, we DESPAWN unused geometry
    // because holding onto unique Material handles in a hidden pool is complex.
    for entity in pool_geo {
        // We must call the specialized cleanup to drop assets
        if let Ok((_, res)) = renderer.geo_resources.q_transient.get(entity) {
            if let Some(h) = &res.mesh { renderer.geo_resources.meshes.remove(h); }
            if let Some(h) = &res.material_2d { renderer.geo_resources.materials_2d.remove(h); }
            if let Some(h) = &res.material_3d { renderer.geo_resources.materials_3d.remove(h); }
        }
        renderer.commands.entity(entity).despawn();
    }

    // Sprites: Safe to Hide
    for entity in pool_sprites {
        renderer.commands.entity(entity).insert(Visibility::Hidden);
    }

    // Text: Safe to Hide
    for entity in pool_text {
        renderer.commands.entity(entity).insert(Visibility::Hidden);
    }

    // Lights: Safe to Hide
    for entity in pool_lights {
        renderer.commands.entity(entity).insert(Visibility::Hidden);
    }
}
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
    pub renderers: ParamSet<'w, 's, (
        GeometryRenderer<'w, 's>, // p0
        SpriteRenderer<'w, 's>,   // p1
        TextRenderer<'w, 's>,     // p2
        LightRenderer<'w, 's>     // p3
    )>
}

pub fn render_graphics(mut renderer: UnifiedRenderer) {

    // 1. PREPARE POOLS OF AVAILABLE ENTITIES

    // Geometry Pool
    let mut pool_geo: Vec<Entity> = renderer.renderers.p0()
        .q_transient.iter()
        .map(|(e, _)| e)
        .collect();

    // Sprite Pool
    let mut pool_sprites: Vec<Entity> = renderer.renderers.p1()
        .q_sprites.iter()
        .map(|(e, ..)| e)
        .collect();

    // Text Pool
    let mut pool_text: Vec<Entity> = renderer.renderers.p2()
        .q_text.iter()
        .map(|(e, ..)| e)
        .collect();

    // Light Pool
    let mut pool_lights: Vec<Entity> = renderer.renderers.p3()
        .q_lights.iter()
        .map(|(e, ..)| e)
        .collect();

    // 2. PROCESS COMMANDS
    let commands_vec: Vec<GraphicsCommand> = renderer.queue.0.drain(..).collect();

    for command in commands_vec {
        match command {
            GraphicsCommand::Geometry(cmd) => {
                let entity = pool_geo.pop();
                let mut geo_system_param = renderer.renderers.p0();
                process_geometry(&mut renderer.commands, &mut geo_system_param, entity, cmd);
            },
            GraphicsCommand::Sprite(cmd) => {
                let entity = pool_sprites.pop();
                let mut sprite_system_param = renderer.renderers.p1();
                process_sprite(&mut renderer.commands, &mut sprite_system_param, entity, cmd);
            },
            GraphicsCommand::Text(cmd) => {
                let entity = pool_text.pop();
                let mut text_system_param = renderer.renderers.p2();
                process_text(&mut renderer.commands, &mut text_system_param, entity, cmd);
            },
            GraphicsCommand::Light(cmd) => {
                let entity = pool_lights.pop();
                let mut light_system_param = renderer.renderers.p3();
                process_light(&mut renderer.commands, &mut light_system_param, entity, cmd);
            }
        }
    }

    // 3. CLEANUP (Recycle)

    // Geometry: Must Despawn
    {
        let mut geo = renderer.renderers.p0();
        for entity in pool_geo {
            // We must call the specialized cleanup to drop assets
            if let Ok((_, res)) = geo.q_transient.get(entity) {
                if let Some(h) = &res.mesh { geo.meshes.remove(h); }
                if let Some(h) = &res.material_2d { geo.materials_2d.remove(h); }
                if let Some(h) = &res.material_3d { geo.materials_3d.remove(h); }
            }
            renderer.commands.entity(entity).despawn();
        }
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
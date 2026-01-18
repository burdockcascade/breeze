use bevy::prelude::*;
use bevy::camera::visibility::RenderLayers;
use bevy::ecs::system::SystemParam;
use std::cell::RefCell;

// Import the Unified Types
use crate::graphics::commands::{GraphicsCommand, GraphicsQueue};

// --- 1. COMMAND DATA ---
#[derive(Clone)]
pub enum LightCommand {
    Point {
        position: Vec3,
        color: Color,
        intensity: f32,
        radius: f32,
        layer: usize,
    },
    Directional {
        direction: Vec3,
        color: Color,
        illuminance: f32,
        layer: usize,
    },
}

// --- 2. MARKER COMPONENT (For Cleanup) ---
#[derive(Component)]
pub struct ImmediateLight;

// --- 3. CONTEXT (Frontend) ---
pub struct LightContext<'a> {
    pub queue: &'a RefCell<&'a mut GraphicsQueue>,
    pub layer_id: usize,
}

impl<'a> LightContext<'a> {

    pub fn point(&self, position: Vec3, color: Color, intensity: f32, radius: f32) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Light(LightCommand::Point {
            position,
            color,
            intensity,
            radius,
            layer: self.layer_id,
        }));
    }

    pub fn directional(&self, direction: Vec3, color: Color, illuminance: f32) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Light(LightCommand::Directional {
            direction,
            color,
            illuminance,
            layer: self.layer_id,
        }));
    }
}

#[derive(SystemParam)]
pub struct LightRenderer<'w, 's> {
    // Query with Option<> to detect which light type exists
    pub q_lights: Query<'w, 's, (
        Entity,
        Option<&'static mut PointLight>,
        Option<&'static mut DirectionalLight>,
        &'static mut Transform,
        &'static mut Visibility,
        &'static mut RenderLayers
    ), With<ImmediateLight>>,
}

// --- 5. PROCESS HELPER ---
pub fn process_light(
    commands: &mut Commands,
    renderer: &mut LightRenderer, // CHANGED: Takes mutable renderer
    entity_opt: Option<Entity>,
    cmd: LightCommand
) {
    // 1. FAST PATH
    if let Some(entity) = entity_opt {
        if let Ok((e, mut pl, mut dl, mut xform, mut vis, mut layers)) = renderer.q_lights.get_mut(entity) {

            // Helper to reset common properties
            *vis = Visibility::Visible;
            *layers = RenderLayers::layer(match cmd {
                LightCommand::Point { layer, .. } => layer,
                LightCommand::Directional { layer, .. } => layer
            });

            match cmd {
                LightCommand::Point { position, color, intensity, radius, .. } => {
                    xform.translation = position;
                    xform.rotation = Quat::IDENTITY;

                    if let Some(ref mut light) = pl {
                        // Recycled correctly
                        light.color = color;
                        light.intensity = intensity;
                        light.range = radius;
                    } else {
                        // Wrong type: Use commands to swap components (cannot update this frame)
                        commands.entity(e)
                            .remove::<DirectionalLight>()
                            .insert(PointLight {
                                color, intensity, range: radius, shadows_enabled: true, ..default()
                            });
                    }
                    return;
                },
                LightCommand::Directional { direction, color, illuminance, .. } => {
                    xform.rotation = Quat::from_rotation_arc(Vec3::NEG_Z, direction.normalize_or_zero());
                    xform.translation = Vec3::ZERO;

                    if let Some(ref mut light) = dl {
                        // Recycled correctly
                        light.color = color;
                        light.illuminance = illuminance;
                    } else {
                        // Wrong type
                        commands.entity(e)
                            .remove::<PointLight>()
                            .insert(DirectionalLight {
                                color, illuminance, shadows_enabled: true, ..default()
                            });
                    }
                    return;
                }
            }
        }
    }

    // 2. SLOW PATH (Spawn New)
    let mut e = commands.spawn(ImmediateLight);

    match cmd {
        LightCommand::Point { position, color, intensity, radius, layer } => {
            e.insert((
                PointLight { color, intensity, range: radius, shadows_enabled: true, ..default() },
                Transform::from_translation(position),
                RenderLayers::layer(layer),
                Visibility::Visible,
            ));
        }
        LightCommand::Directional { direction, color, illuminance, layer } => {
            let rotation = Quat::from_rotation_arc(Vec3::NEG_Z, direction.normalize_or_zero());
            e.insert((
                DirectionalLight { color, illuminance, shadows_enabled: true, ..default() },
                Transform::from_rotation(rotation),
                RenderLayers::layer(layer),
                Visibility::Visible,
            ));
        }
    }
}
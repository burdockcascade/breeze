use bevy::prelude::*;
use bevy::camera::visibility::RenderLayers;
use bevy::ecs::system::SystemParam;
use std::cell::RefCell;

use crate::graphics::commands::{GraphicsCommand, GraphicsQueue};

#[derive(Clone)]
pub enum LightCommand {
    Point {
        position: Vec3,
        color: Color,
        intensity: f32,
        radius: f32,
        shadows: bool,
        layer: usize,
    },
    Directional {
        direction: Vec3,
        color: Color,
        illuminance: f32,
        shadows: bool,
        layer: usize,
    },
}

#[derive(Component)]
pub struct ImmediateLight;

pub struct LightContext<'a> {
    pub queue: &'a RefCell<&'a mut GraphicsQueue>,
    pub layer_id: usize,
}

impl<'a> LightContext<'a> {

    /// Create a point light with the specified position, color, intensity, radius, and shadows.
    pub fn point(&self, position: Vec3, color: Color, intensity: f32, radius: f32, shadows: bool) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Light(LightCommand::Point {
            position,
            color,
            intensity,
            radius,
            shadows,
            layer: self.layer_id,
        }));
    }

    /// Create a directional light with the specified direction, color, illuminance, and shadows.
    pub fn directional(&self, direction: Vec3, color: Color, illuminance: f32, shadows: bool) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Light(LightCommand::Directional {
            direction,
            color,
            illuminance,
            shadows,
            layer: self.layer_id,
        }));
    }
}

#[derive(SystemParam)]
pub struct LightRenderer<'w, 's> {
    pub q_lights: Query<'w, 's, (
        Entity,
        Option<&'static mut PointLight>,
        Option<&'static mut DirectionalLight>,
        &'static mut Transform,
        &'static mut Visibility,
        &'static mut RenderLayers
    ), With<ImmediateLight>>,
}

/// Process a light command, updating an existing light or spawning a new one.
pub fn process_light(commands: &mut Commands, renderer: &mut LightRenderer, entity_opt: Option<Entity>, cmd: LightCommand) {
    
    if let Some(entity) = entity_opt {
        if let Ok((e, mut pl, mut dl, mut xform, mut vis, mut layers)) = renderer.q_lights.get_mut(entity) {

            *vis = Visibility::Visible;
            *layers = RenderLayers::layer(match cmd {
                LightCommand::Point { layer, .. } => layer,
                LightCommand::Directional { layer, .. } => layer
            });

            match cmd {
                LightCommand::Point { position, color, intensity, radius, shadows, .. } => {
                    xform.translation = position;
                    xform.rotation = Quat::IDENTITY;

                    if let Some(ref mut light) = pl {
                        light.color = color;
                        light.intensity = intensity;
                        light.range = radius;
                        light.shadows_enabled = shadows;
                    } else {
                        commands.entity(e)
                            .remove::<DirectionalLight>()
                            .insert(PointLight {
                                color, intensity, range: radius, shadows_enabled: shadows, ..default()
                            });
                    }
                    return;
                },
                LightCommand::Directional { direction, color, illuminance, shadows, .. } => {
                    xform.rotation = Quat::from_rotation_arc(Vec3::NEG_Z, direction.normalize_or_zero());
                    xform.translation = Vec3::ZERO;

                    if let Some(ref mut light) = dl {
                        light.color = color;
                        light.illuminance = illuminance;
                        light.shadows_enabled = shadows; // UPDATE SHADOWS
                    } else {
                        commands.entity(e)
                            .remove::<PointLight>()
                            .insert(DirectionalLight {
                                color, illuminance, shadows_enabled: shadows, ..default()
                            });
                    }
                    return;
                }
            }
        }
    }

    let mut e = commands.spawn(ImmediateLight);

    match cmd {
        LightCommand::Point { position, color, intensity, radius, shadows, layer } => {
            e.insert((
                PointLight { color, intensity, range: radius, shadows_enabled: shadows, ..default() },
                Transform::from_translation(position),
                RenderLayers::layer(layer),
                Visibility::Visible,
            ));
        }
        LightCommand::Directional { direction, color, illuminance, shadows, layer } => {
            let rotation = Quat::from_rotation_arc(Vec3::NEG_Z, direction.normalize_or_zero());
            e.insert((
                DirectionalLight { color, illuminance, shadows_enabled: shadows, ..default() },
                Transform::from_rotation(rotation),
                RenderLayers::layer(layer),
                Visibility::Visible,
            ));
        }
    }
}
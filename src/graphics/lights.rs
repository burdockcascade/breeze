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

// --- 4. RENDERER RESOURCES (Backend) ---
#[derive(SystemParam)]
pub struct LightRenderer<'w, 's> {
    // Query to find lights from the previous frame for cleanup
    pub q_lights: Query<'w, 's, Entity, With<ImmediateLight>>,
}

// --- 5. SPAWN HELPER (Called by UnifiedRenderer) ---
pub fn spawn_light(
    commands: &mut Commands,
    _renderer: &LightRenderer,
    cmd: LightCommand
) {
    match cmd {
        LightCommand::Point { position, color, intensity, radius, layer } => {
            commands.spawn((
                PointLight {
                    color,
                    intensity,
                    range: radius,
                    shadows_enabled: true, // Default to true for Breeze
                    ..default()
                },
                Transform::from_translation(position),
                RenderLayers::layer(layer),
                Visibility::Visible,
                ImmediateLight, // Mark for cleanup
            ));
        }
        LightCommand::Directional { direction, color, illuminance, layer } => {
            // Directional lights need a rotation to define direction
            // We look towards the direction from zero
            let rotation = Quat::from_rotation_arc(Vec3::NEG_Z, direction.normalize_or_zero());

            commands.spawn((
                DirectionalLight {
                    color,
                    illuminance,
                    shadows_enabled: true,
                    ..default()
                },
                Transform::from_rotation(rotation),
                RenderLayers::layer(layer),
                Visibility::Visible,
                ImmediateLight, // Mark for cleanup
            ));
        }
    }
}
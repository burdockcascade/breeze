// src/graphics/lights.rs
use bevy::prelude::*;
use bevy::camera::visibility::RenderLayers;
use bevy::ecs::system::SystemParam;

// 1. Independent Commands
#[derive(Clone, Copy)]
pub enum LightCommand {
    Point { position: Vec3, color: Color, intensity: f32, radius: f32, layer: usize },
    Directional { direction: Vec3, color: Color, illuminance: f32, layer: usize },
}

// 2. Independent Queue
#[derive(Resource, Default)]
pub struct LightQueue(pub Vec<LightCommand>);

// 3. Independent Context (Frontend)
pub struct LightContext<'a> {
    pub queue: &'a mut LightQueue,
    pub layer_id: usize,
}

impl<'a> LightContext<'a> {
    pub fn new(queue: &'a mut LightQueue, layer_id: usize) -> Self {
        Self { queue, layer_id }
    }

    pub fn point(&mut self, position: Vec3, color: Color, intensity: f32, radius: f32) {
        self.queue.0.push(LightCommand::Point {
            position,
            color,
            intensity,
            radius,
            layer: self.layer_id
        });
    }

    pub fn directional(&mut self, direction: Vec3, color: Color, illuminance: f32) {
        self.queue.0.push(LightCommand::Directional {
            direction,
            color,
            illuminance,
            layer: self.layer_id
        });
    }
}

// --- RENDERING & POOLING LOGIC ---

#[derive(Component)]
pub struct PooledLight {
    pub light_type: LightType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    Point,
    Directional,
}

#[derive(SystemParam)]
pub struct LightRenderer<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub queue: ResMut<'w, LightQueue>,

    // Queries to reuse existing lights
    pub q_point_lights: Query<'w, 's,
        (Entity, &'static mut PointLight, &'static mut Transform, &'static mut Visibility, &'static PooledLight),
        Without<DirectionalLight>
    >,

    pub q_dir_lights: Query<'w, 's,
        (Entity, &'static mut DirectionalLight, &'static mut Transform, &'static mut Visibility, &'static PooledLight),
        Without<PointLight>
    >,
}

pub fn render_lights(mut renderer: LightRenderer) {
    // 1. Gather available lights for pooling
    let mut available_point_lights: Vec<Entity> = renderer.q_point_lights.iter()
        .filter(|(.., pool)| pool.light_type == LightType::Point)
        .map(|(e, ..)| e)
        .collect();

    let mut available_dir_lights: Vec<Entity> = renderer.q_dir_lights.iter()
        .filter(|(.., pool)| pool.light_type == LightType::Directional)
        .map(|(e, ..)| e)
        .collect();

    // 2. Process Queue
    for command in renderer.queue.0.drain(..) {
        match command {
            LightCommand::Point { position, color, intensity, radius, layer } => {
                if let Some(entity) = available_point_lights.pop() {
                    // Reuse
                    let (_, mut light, mut transform, mut visibility, _) = renderer.q_point_lights.get_mut(entity).unwrap();
                    light.color = color;
                    light.intensity = intensity;
                    light.range = radius;
                    light.shadows_enabled = true;
                    *transform = Transform::from_translation(position);
                    *visibility = Visibility::Visible;
                } else {
                    // Create
                    renderer.commands.spawn((
                        PointLight {
                            color,
                            intensity,
                            range: radius,
                            shadows_enabled: true,
                            ..default()
                        },
                        Transform::from_translation(position),
                        RenderLayers::layer(layer),
                        PooledLight { light_type: LightType::Point },
                    ));
                }
            }
            LightCommand::Directional { direction, color, illuminance, layer } => {
                let rotation = Quat::from_rotation_arc(Vec3::NEG_Z, direction.normalize_or_zero());

                if let Some(entity) = available_dir_lights.pop() {
                    // Reuse
                    let (_, mut light, mut transform, mut visibility, _) = renderer.q_dir_lights.get_mut(entity).unwrap();
                    light.color = color;
                    light.illuminance = illuminance;
                    light.shadows_enabled = true;
                    *transform = Transform::from_rotation(rotation);
                    *visibility = Visibility::Visible;
                } else {
                    // Create
                    renderer.commands.spawn((
                        DirectionalLight {
                            color,
                            illuminance,
                            shadows_enabled: true,
                            ..default()
                        },
                        Transform::from_rotation(rotation),
                        RenderLayers::layer(layer),
                        PooledLight { light_type: LightType::Directional },
                    ));
                }
            }
        }
    }

    // 3. Hide unused lights (Pooling)
    for entity in available_point_lights {
        if let Ok((_, _, _, mut vis, _)) = renderer.q_point_lights.get_mut(entity) {
            *vis = Visibility::Hidden;
        }
    }

    for entity in available_dir_lights {
        if let Ok((_, _, _, mut vis, _)) = renderer.q_dir_lights.get_mut(entity) {
            *vis = Visibility::Hidden;
        }
    }
}
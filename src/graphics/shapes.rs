use bevy::prelude::*;
use bevy::camera::visibility::RenderLayers;
use bevy::ecs::system::SystemParam;

/// Resource to store the Unit meshes so we don't recreate them every frame.
#[derive(Resource)]
pub struct GlobalShapeResources {
    // 2D
    pub circle: Handle<Mesh>,
    pub rect: Handle<Mesh>,
    // 3D
    pub cuboid: Handle<Mesh>,
    pub sphere: Handle<Mesh>,
    pub cylinder: Handle<Mesh>,
    pub cone: Handle<Mesh>,
    pub taurus: Handle<Mesh>,
    pub plane: Handle<Mesh>,
}

impl FromWorld for GlobalShapeResources {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        Self {
            circle: meshes.add(Circle::new(1.0)),
            rect: meshes.add(Rectangle::new(1.0, 1.0)),
            cuboid: meshes.add(Cuboid::from_length(1.0)),
            sphere: meshes.add(Sphere::new(1.0)),
            cylinder: meshes.add(Cylinder::new(1.0, 1.0)),
            cone: meshes.add(Cone::new(1.0, 1.0).mesh()),
            taurus: meshes.add(Torus::new(1.0, 0.3).mesh()),
            plane: meshes.add(Plane3d::default().mesh().size(1.0, 1.0)),
        }
    }
}

/// Commands to be executed by the renderer
#[derive(Clone, Copy)]
pub enum ShapeCommand {
    // 2D Commands
    Circle { position: Vec2, radius: f32, color: Color, layer: usize },
    Rect { position: Vec2, size: Vec2, color: Color, layer: usize },
    Line { start: Vec2, end: Vec2, thickness: f32, color: Color, layer: usize },
    Ring { position: Vec2, radius: f32, thickness: f32, color: Color, layer: usize },

    // 3D Commands
    Cube { position: Vec3, rotation: Quat, size: f32, color: Color, layer: usize },
    Cuboid { position: Vec3, rotation: Quat, size: Vec3, color: Color, layer: usize },
    Sphere { position: Vec3, radius: f32, color: Color, layer: usize },
    Cylinder { position: Vec3, rotation: Quat, radius: f32, height: f32, color: Color, layer: usize },
    Cone { position: Vec3, rotation: Quat, radius: f32, height: f32, color: Color, layer: usize },
    Taurus { position: Vec3, rotation: Quat, radius: f32, tube_radius: f32, color: Color, layer: usize },
    Plane { position: Vec3, rotation: Quat, size: f32, color: Color, layer: usize },

    // Light Commands
    PointLight { position: Vec3, color: Color, intensity: f32, radius: f32, layer: usize },
    DirectionalLight { direction: Vec3, color: Color, illuminance: f32, layer: usize },
}

#[derive(Component)]
pub struct PooledLight {
    pub light_type: LightType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LightType {
    Point,
    Directional,
}

/// Resource to queue up shape commands for the current frame
#[derive(Resource, Default)]
pub struct ShapeQueue(pub Vec<ShapeCommand>);

/// Component to tag shapes that should be despawned at the end of the frame
#[derive(Component)]
pub struct TransientResources {
    pub mesh: Option<Handle<Mesh>>,
    pub material_2d: Option<Handle<ColorMaterial>>,
    pub material_3d: Option<Handle<StandardMaterial>>,
}

pub struct ShapeContext<'a> {
    pub queue: &'a mut ShapeQueue,
    pub layer_id: usize,
}

impl<'a> ShapeContext<'a> {
    pub fn new(queue: &'a mut ShapeQueue, layer_id: usize) -> Self {
        Self { queue, layer_id }
    }

    /// Draw a circle at the given position, radius, and color
    pub fn circle(&mut self, position: Vec2, radius: f32, color: Color) {
        self.queue.0.push(ShapeCommand::Circle { position, radius, color, layer: self.layer_id });
    }

    /// Draw a rectangle at the given position, size, and color
    pub fn rect(&mut self, position: Vec2, size: Vec2, color: Color) {
        self.queue.0.push(ShapeCommand::Rect { position, size, color, layer: self.layer_id });
    }

    /// Draw a line from start to end with given thickness and color
    pub fn line(&mut self, start: Vec2, end: Vec2, thickness: f32, color: Color) {
        self.queue.0.push(ShapeCommand::Line { start, end, thickness, color, layer: self.layer_id });
    }

    /// Draw a ring at the given position, radius, thickness, and color
    pub fn ring(&mut self, position: Vec2, radius: f32, thickness: f32, color: Color) {
        self.queue.0.push(ShapeCommand::Ring { position, radius, thickness, color, layer: self.layer_id });
    }

    /// Draw a cube at the given position, rotation, size, and color
    pub fn cube(&mut self, position: Vec3, rotation: Quat, size: f32, color: Color) {
        self.queue.0.push(ShapeCommand::Cube { position, rotation, size, color, layer: self.layer_id });
    }

    /// Draw a cuboid at the given position, rotation, size, and color
    pub fn cuboid(&mut self, position: Vec3, rotation: Quat, size: Vec3, color: Color) {
        self.queue.0.push(ShapeCommand::Cuboid { position, rotation, size, color, layer: self.layer_id });
    }

    /// Draw a sphere at the given position, radius, and color
    pub fn sphere(&mut self, position: Vec3, radius: f32, color: Color) {
        self.queue.0.push(ShapeCommand::Sphere { position, radius, color, layer: self.layer_id });
    }

    /// Draw a cylinder at the given position, rotation, radius, height, and color
    pub fn cylinder(&mut self, position: Vec3, rotation: Quat, radius: f32, height: f32, color: Color) {
        self.queue.0.push(ShapeCommand::Cylinder { position, rotation, radius, height, color, layer: self.layer_id });
    }

    /// Draw a cone at the given position, rotation, radius, height, and color
    pub fn cone(&mut self, position: Vec3, rotation: Quat, radius: f32, height: f32, color: Color) {
        self.queue.0.push(ShapeCommand::Cone { position, rotation, radius, height, color, layer: self.layer_id });
    }

    /// Draw a taurus at the given position, rotation, radius, tube_radius, and color
    pub fn torus(&mut self, position: Vec3, rotation: Quat, radius: f32, tube_radius: f32, color: Color) {
        self.queue.0.push(ShapeCommand::Taurus { position, rotation, radius, tube_radius, color, layer: self.layer_id });
    }

    /// Draw a plane at the given position, rotation, size, and color
    pub fn plane(&mut self, position: Vec3, rotation: Quat, size: f32, color: Color) {
        self.queue.0.push(ShapeCommand::Plane { position, rotation, size, color, layer: self.layer_id });
    }

    pub fn point_light(&mut self, position: Vec3, color: Color, intensity: f32, radius: f32) {
        self.queue.0.push(ShapeCommand::PointLight {
            position,
            color,
            intensity,
            radius,
            layer: self.layer_id
        });
    }

    pub fn directional_light(&mut self, direction: Vec3, color: Color, illuminance: f32) {
        self.queue.0.push(ShapeCommand::DirectionalLight {
            direction,
            color,
            illuminance,
            layer: self.layer_id
        });
    }
}

#[derive(SystemParam)]
pub struct ShapeRenderer<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub queue: ResMut<'w, ShapeQueue>,
    pub global_shapes: Res<'w, GlobalShapeResources>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials_2d: ResMut<'w, Assets<ColorMaterial>>,
    // Include this if you added the 3D support
    pub materials_3d: ResMut<'w, Assets<StandardMaterial>>,
    pub q_transient: Query<'w, 's, (Entity, &'static TransientResources)>,

    // --- FIXED QUERIES ---
    // We add "Without<DirectionalLight>" to the PointLight query
    pub q_point_lights: Query<'w, 's,
        (Entity, &'static mut PointLight, &'static mut Transform, &'static mut Visibility, &'static PooledLight),
        Without<DirectionalLight>
    >,

    // We add "Without<PointLight>" to the DirectionalLight query
    pub q_dir_lights: Query<'w, 's,
        (Entity, &'static mut DirectionalLight, &'static mut Transform, &'static mut Visibility, &'static PooledLight),
        Without<PointLight>
    >,
}

pub fn render_shapes(mut renderer: ShapeRenderer) {

    // Clean up previous frame's transient shapes
    for (entity, resources) in renderer.q_transient.iter() {
        if let Some(handle) = &resources.mesh {
            renderer.meshes.remove(handle);
        }
        if let Some(handle) = &resources.material_2d {
            renderer.materials_2d.remove(handle);
        }
        if let Some(handle) = &resources.material_3d {
            renderer.materials_3d.remove(handle);
        }
        renderer.commands.entity(entity).despawn();
    }

    let mut available_point_lights: Vec<Entity> = renderer.q_point_lights.iter()
        .filter(|(.., pool)| pool.light_type == LightType::Point)
        .map(|(e, ..)| e)
        .collect();

    let mut available_dir_lights: Vec<Entity> = renderer.q_dir_lights.iter()
        .filter(|(.., pool)| pool.light_type == LightType::Directional)
        .map(|(e, ..)| e)
        .collect();

    // Process the queue
    for command in renderer.queue.0.drain(..) {
        match command {
            // --- 2D Rendering ---
            ShapeCommand::Circle { position, radius, color, layer } => {
                let material = renderer.materials_2d.add(ColorMaterial::from(color));
                renderer.commands.spawn((
                    Mesh2d(renderer.global_shapes.circle.clone()),
                    MeshMaterial2d(material.clone()),
                    Transform::from_translation(position.extend(0.0)).with_scale(Vec3::splat(radius)),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: None, material_2d: Some(material), material_3d: None },
                ));
            }
            ShapeCommand::Rect { position, size, color, layer } => {
                let material = renderer.materials_2d.add(ColorMaterial::from(color));
                renderer.commands.spawn((
                    Mesh2d(renderer.global_shapes.rect.clone()),
                    MeshMaterial2d(material.clone()),
                    Transform::from_translation(position.extend(0.0)).with_scale(size.extend(1.0)),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: None, material_2d: Some(material), material_3d: None },
                ));
            }
            ShapeCommand::Line { start, end, thickness, color, layer } => {
                let center = (start + end) / 2.0;
                let length = start.distance(end);
                let angle = (end.y - start.y).atan2(end.x - start.x);

                let material = renderer.materials_2d.add(ColorMaterial::from(color));
                renderer.commands.spawn((
                    Mesh2d(renderer.global_shapes.rect.clone()),
                    MeshMaterial2d(material.clone()),
                    Transform::from_translation(center.extend(0.0))
                        .with_rotation(Quat::from_rotation_z(angle))
                        .with_scale(Vec3::new(length, thickness, 1.0)),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: None, material_2d: Some(material), material_3d: None },
                ));
            }
            ShapeCommand::Ring { position, radius, thickness, color, layer } => {
                let inner = radius - thickness / 2.0;
                let outer = radius + thickness / 2.0;
                let mesh_handle = renderer.meshes.add(Annulus::new(inner, outer));
                let mat_handle = renderer.materials_2d.add(ColorMaterial::from(color));

                renderer.commands.spawn((
                    Mesh2d(mesh_handle.clone()),
                    MeshMaterial2d(mat_handle.clone()),
                    Transform::from_translation(position.extend(0.0)),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: Some(mesh_handle), material_2d: Some(mat_handle), material_3d: None },
                ));
            }

            // --- 3D Rendering ---
            ShapeCommand::Cube { position, rotation, size, color, layer } => {
                let material = renderer.materials_3d.add(StandardMaterial::from(color));
                renderer.commands.spawn((
                    Mesh3d(renderer.global_shapes.cuboid.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(position).with_rotation(rotation).with_scale(Vec3::splat(size)),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
                ));
            }
            ShapeCommand::Cuboid { position, rotation, size, color, layer } => {
                let material = renderer.materials_3d.add(StandardMaterial::from(color));
                renderer.commands.spawn((
                    Mesh3d(renderer.global_shapes.cuboid.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(position).with_rotation(rotation).with_scale(size),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
                ));
            }
            ShapeCommand::Sphere { position, radius, color, layer } => {
                let material = renderer.materials_3d.add(StandardMaterial::from(color));
                renderer.commands.spawn((
                    Mesh3d(renderer.global_shapes.sphere.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(position).with_scale(Vec3::splat(radius)),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
                ));
            }
            ShapeCommand::Cylinder { position, rotation, radius, height, color, layer } => {
                let material = renderer.materials_3d.add(StandardMaterial::from(color));
                renderer.commands.spawn((
                    Mesh3d(renderer.global_shapes.cylinder.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(position).with_rotation(rotation).with_scale(Vec3::new(radius, height, radius)),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
                ));
            }
            ShapeCommand::Cone { position, rotation, radius, height, color, layer } => {
                let material = renderer.materials_3d.add(StandardMaterial::from(color));
                renderer.commands.spawn((
                    Mesh3d(renderer.global_shapes.cone.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(position).with_rotation(rotation),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
                ));
            }
            ShapeCommand::Taurus { position, rotation, radius, tube_radius, color, layer } => {
                let material = renderer.materials_3d.add(StandardMaterial::from(color));
                renderer.commands.spawn((
                    Mesh3d(renderer.global_shapes.taurus.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(position).with_rotation(rotation),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
                ));
            }
            ShapeCommand::Plane { position, rotation, size, color, layer } => {
                let material = renderer.materials_3d.add(StandardMaterial::from(color));
                renderer.commands.spawn((
                    Mesh3d(renderer.global_shapes.plane.clone()),
                    MeshMaterial3d(material.clone()),
                    Transform::from_translation(position).with_rotation(rotation).with_scale(Vec3::new(size, 1.0, size)),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
                ));
            }
            ShapeCommand::PointLight { position, color, intensity, radius, layer } => {
                if let Some(entity) = available_point_lights.pop() {
                    // REUSE: Update existing entity
                    let (_, mut light, mut transform, mut visibility, _) = renderer.q_point_lights.get_mut(entity).unwrap();
                    light.color = color;
                    light.intensity = intensity;
                    light.range = radius;
                    light.shadows_enabled = true; // CAUTION: Shadows are still expensive!
                    *transform = Transform::from_translation(position);
                    *visibility = Visibility::Visible;
                } else {
                    // CREATE: Spawn new if pool is empty
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
                        PooledLight { light_type: LightType::Point }, // Mark for pooling
                        // Note: Do NOT add TransientResources here
                    ));
                }
            }
            ShapeCommand::DirectionalLight { direction, color, illuminance, layer } => {
                let rotation = Quat::from_rotation_arc(Vec3::NEG_Z, direction.normalize_or_zero());

                if let Some(entity) = available_dir_lights.pop() {
                    // REUSE
                    let (_, mut light, mut transform, mut visibility, _) = renderer.q_dir_lights.get_mut(entity).unwrap();
                    light.color = color;
                    light.illuminance = illuminance;
                    light.shadows_enabled = true;
                    *transform = Transform::from_rotation(rotation);
                    *visibility = Visibility::Visible;
                } else {
                    // CREATE
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
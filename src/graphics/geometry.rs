use std::cell::RefCell;
use bevy::prelude::*;
use bevy::camera::visibility::RenderLayers;
use bevy::ecs::system::SystemParam;
use crate::graphics::commands::{GraphicsCommand, GraphicsQueue};
// =================================================================================
//  BACKEND: RESOURCES & COMMANDS
// =================================================================================

/// Shared resource to store the Unit meshes so we don't recreate them every frame.
#[derive(Resource)]
pub struct GlobalGeometryResources {
    // 2D Meshes
    pub circle: Handle<Mesh>,
    pub rect: Handle<Mesh>,
    // 3D Meshes
    pub cuboid: Handle<Mesh>,
    pub sphere: Handle<Mesh>,
    pub cylinder: Handle<Mesh>,
    pub cone: Handle<Mesh>,
    pub torus: Handle<Mesh>,
    pub plane: Handle<Mesh>,
}

impl FromWorld for GlobalGeometryResources {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        Self {
            circle: meshes.add(Circle::new(1.0)),
            rect: meshes.add(Rectangle::new(1.0, 1.0)),
            cuboid: meshes.add(Cuboid::from_length(1.0)),
            sphere: meshes.add(Sphere::new(1.0)),
            cylinder: meshes.add(Cylinder::new(1.0, 1.0)),
            cone: meshes.add(Cone::new(1.0, 1.0).mesh()),
            torus: meshes.add(Torus::new(1.0, 0.3).mesh()),
            plane: meshes.add(Plane3d::default().mesh().size(1.0, 1.0)),
        }
    }
}

/// A unified command queue for all geometry.
#[derive(Resource, Default)]
pub struct GeometryQueue(pub Vec<GeometryCommand>);

/// Commands to be executed by the renderer.
/// We split variants by functionality, but they live in the same enum.
#[derive(Clone)] // Clone is needed for Handle<Scene>
pub enum GeometryCommand {
    // --- UNLIT 2D (Uses ColorMaterial) ---
    Circle { position: Vec2, radius: f32, color: Color, layer: usize },
    Rect { position: Vec2, size: Vec2, color: Color, layer: usize },
    Line { start: Vec2, end: Vec2, thickness: f32, color: Color, layer: usize },
    Ring { position: Vec2, radius: f32, thickness: f32, color: Color, layer: usize },

    // --- LIT 3D & LIT 2D (Uses StandardMaterial) ---
    Cube { position: Vec3, rotation: Quat, size: f32, color: Color, layer: usize },
    Cuboid { position: Vec3, rotation: Quat, size: Vec3, color: Color, layer: usize },
    Sphere { position: Vec3, radius: f32, color: Color, layer: usize },
    Cylinder { position: Vec3, rotation: Quat, radius: f32, height: f32, color: Color, layer: usize },
    Cone { position: Vec3, rotation: Quat, radius: f32, height: f32, color: Color, layer: usize },
    Torus { position: Vec3, rotation: Quat, radius: f32, tube_radius: f32, color: Color, layer: usize },
    Plane { position: Vec3, rotation: Quat, size: f32, color: Color, layer: usize },

    // "Lit 2D" helper (Just a plane facing the camera)
    Quad { position: Vec3, rotation: Quat, size: Vec2, color: Color, layer: usize },

    // Imported 3D Models
    Model { position: Vec3, rotation: Quat, scale: Vec3, scene: Handle<Scene>, layer: usize },
}

// =================================================================================
//  FRONTEND 1: UNLIT 2D (UI, HUD, Flat Overlays)
// =================================================================================

pub struct Geometry2d<'a> {
    // Holds a reference to a Cell containing the mutable queue
    pub queue: &'a RefCell<&'a mut GraphicsQueue>,
    pub layer_id: usize,
}

impl<'a> Geometry2d<'a> {
    pub fn circle(&self, position: Vec2, radius: f32, color: Color) {
        self.queue.borrow_mut().0.push(
            GraphicsCommand::Geometry(
                GeometryCommand::Circle { position, radius, color, layer: self.layer_id }
            )
        );
    }

    pub fn rect(&self, position: Vec2, size: Vec2, color: Color) {
        self.queue.borrow_mut().0.push(
            GraphicsCommand::Geometry(
                GeometryCommand::Rect { position, size, color, layer: self.layer_id }
            )
        );
    }

    pub fn line(&self, start: Vec2, end: Vec2, thickness: f32, color: Color) {
        self.queue.borrow_mut().0.push(
            GraphicsCommand::Geometry(
                GeometryCommand::Line { start, end, thickness, color, layer: self.layer_id }
            )
        );
    }

    pub fn ring(&self, position: Vec2, radius: f32, thickness: f32, color: Color) {
        self.queue.borrow_mut().0.push(
            GraphicsCommand::Geometry(
                GeometryCommand::Ring { position, radius, thickness, color, layer: self.layer_id }
            )
        );
    }
}

pub struct Geometry3d<'a> {
    pub queue: &'a RefCell<&'a mut GraphicsQueue>,
    pub layer_id: usize,
}

impl<'a> Geometry3d<'a> {
    pub fn cube(&self, position: Vec3, rotation: Quat, size: f32, color: Color) {
        self.queue.borrow_mut().0.push(
            GraphicsCommand::Geometry(
                GeometryCommand::Cube { position, rotation, size, color, layer: self.layer_id }
            )
        );
    }

    pub fn cuboid(&self, position: Vec3, rotation: Quat, size: Vec3, color: Color) {
        self.queue.borrow_mut().0.push(
            GraphicsCommand::Geometry(
                GeometryCommand::Cuboid { position, rotation, size, color, layer: self.layer_id }
            )
        );
    }

    pub fn sphere(&self, position: Vec3, radius: f32, color: Color) {
        self.queue.borrow_mut().0.push(
            GraphicsCommand::Geometry(
                GeometryCommand::Sphere { position, radius, color, layer: self.layer_id }
            )
        );
    }

    pub fn cylinder(&self, position: Vec3, rotation: Quat, radius: f32, height: f32, color: Color) {
        self.queue.borrow_mut().0.push(
            GraphicsCommand::Geometry(
                GeometryCommand::Cylinder { position, rotation, radius, height, color, layer: self.layer_id }
            )
        );
    }

    pub fn cone(&self, position: Vec3, rotation: Quat, radius: f32, height: f32, color: Color) {
        self.queue.borrow_mut().0.push(
            GraphicsCommand::Geometry(
                GeometryCommand::Cone { position, rotation, radius, height, color, layer: self.layer_id }
            )
        );
    }

    pub fn torus(&self, position: Vec3, rotation: Quat, radius: f32, tube_radius: f32, color: Color) {
        self.queue.borrow_mut().0.push(
            GraphicsCommand::Geometry(
                GeometryCommand::Torus { position, rotation, radius, tube_radius, color, layer: self.layer_id }
            )
        );
    }

    pub fn plane(&self, position: Vec3, rotation: Quat, size: f32, color: Color) {
        self.queue.borrow_mut().0.push(
            GraphicsCommand::Geometry(
                GeometryCommand::Plane { position, rotation, size, color, layer: self.layer_id }
            )
        );
    }

    pub fn quad(&self, position: Vec3, rotation: Quat, size: Vec2, color: Color) {
        self.queue.borrow_mut().0.push(
            GraphicsCommand::Geometry(
                GeometryCommand::Quad { position, rotation, size, color, layer: self.layer_id }
            )
        );
    }

    pub fn model(&self, position: Vec3, rotation: Quat, scale: Vec3, scene: Handle<Scene>) {
        self.queue.borrow_mut().0.push(
            GraphicsCommand::Geometry(
                GeometryCommand::Model { position, rotation, scale, scene, layer: self.layer_id }
            )
        );
    }
}

// =================================================================================
//  RENDERER SYSTEM
// =================================================================================

/// Component to tag shapes that should be despawned at the end of the frame
#[derive(Component)]
pub struct TransientResources {
    pub mesh: Option<Handle<Mesh>>,
    pub material_2d: Option<Handle<ColorMaterial>>,
    pub material_3d: Option<Handle<StandardMaterial>>,
}

#[derive(SystemParam)]
pub struct GeometryRenderer<'w, 's> {
    pub global_geo: Res<'w, GlobalGeometryResources>,
    pub meshes: ResMut<'w, Assets<Mesh>>,
    pub materials_2d: ResMut<'w, Assets<ColorMaterial>>,
    pub materials_3d: ResMut<'w, Assets<StandardMaterial>>,
    pub q_transient: Query<'w, 's, (Entity, &'static TransientResources)>,
}

/// Helper function to spawn a single geometry command.
/// This is used by the UnifiedRenderer to process commands from the global queue.
pub fn process_geometry(
    commands: &mut Commands,
    resources: &mut GeometryRenderer,
    entity_opt: Option<Entity>,
    command: GeometryCommand
) {
    // 1. CLEANUP OLD ASSETS (If reusing an entity)
    if let Some(entity) = entity_opt {
        if let Ok((_, old_res)) = resources.q_transient.get(entity) {
            if let Some(handle) = &old_res.mesh { resources.meshes.remove(handle); }
            if let Some(handle) = &old_res.material_2d { resources.materials_2d.remove(handle); }
            if let Some(handle) = &old_res.material_3d { resources.materials_3d.remove(handle); }
        }

        // Remove old components so we don't end up with a Mesh2d AND a Mesh3d on the same entity
        commands.entity(entity)
            .remove::<Mesh2d>()
            .remove::<MeshMaterial2d<ColorMaterial>>()
            .remove::<Mesh3d>()
            .remove::<MeshMaterial3d<StandardMaterial>>()
            .remove::<SceneRoot>();
    }

    // 2. PREPARE ENTITY (Recycle or Spawn)
    let mut cmd_entity = if let Some(e) = entity_opt {
        commands.entity(e)
    } else {
        commands.spawn(())
    };

    // 3. INSERT BUNDLES (Using cmd_entity!)
    match command {
        // --- UNLIT 2D ---
        GeometryCommand::Circle { position, radius, color, layer } => {
            let material = resources.materials_2d.add(ColorMaterial::from(color));
            cmd_entity.insert((
                Mesh2d(resources.global_geo.circle.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_translation(position.extend(0.0)).with_scale(Vec3::splat(radius)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: Some(material), material_3d: None },
            ));
        }
        GeometryCommand::Rect { position, size, color, layer } => {
            let material = resources.materials_2d.add(ColorMaterial::from(color));
            cmd_entity.insert((
                Mesh2d(resources.global_geo.rect.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_translation(position.extend(0.0)).with_scale(size.extend(1.0)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: Some(material), material_3d: None },
            ));
        }
        GeometryCommand::Line { start, end, thickness, color, layer } => {
            let center = (start + end) / 2.0;
            let length = start.distance(end);
            let angle = (end.y - start.y).atan2(end.x - start.x);
            let material = resources.materials_2d.add(ColorMaterial::from(color));

            cmd_entity.insert((
                Mesh2d(resources.global_geo.rect.clone()),
                MeshMaterial2d(material.clone()),
                Transform::from_translation(center.extend(0.0))
                    .with_rotation(Quat::from_rotation_z(angle))
                    .with_scale(Vec3::new(length, thickness, 1.0)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: Some(material), material_3d: None },
            ));
        }
        GeometryCommand::Ring { position, radius, thickness, color, layer } => {
            let inner = radius - thickness / 2.0;
            let outer = radius + thickness / 2.0;
            let mesh_handle = resources.meshes.add(Annulus::new(inner, outer));
            let mat_handle = resources.materials_2d.add(ColorMaterial::from(color));

            cmd_entity.insert((
                Mesh2d(mesh_handle.clone()),
                MeshMaterial2d(mat_handle.clone()),
                Transform::from_translation(position.extend(0.0)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: Some(mesh_handle), material_2d: Some(mat_handle), material_3d: None },
            ));
        }

        // --- LIT 3D ---
        GeometryCommand::Cube { position, rotation, size, color, layer } => {
            let material = resources.materials_3d.add(StandardMaterial::from(color));
            cmd_entity.insert((
                Mesh3d(resources.global_geo.cuboid.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(position).with_rotation(rotation).with_scale(Vec3::splat(size)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
            ));
        }
        GeometryCommand::Cuboid { position, rotation, size, color, layer } => {
            let material = resources.materials_3d.add(StandardMaterial::from(color));
            cmd_entity.insert((
                Mesh3d(resources.global_geo.cuboid.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(position).with_rotation(rotation).with_scale(size),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
            ));
        }
        GeometryCommand::Sphere { position, radius, color, layer } => {
            let material = resources.materials_3d.add(StandardMaterial::from(color));
            cmd_entity.insert((
                Mesh3d(resources.global_geo.sphere.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(position).with_scale(Vec3::splat(radius)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
            ));
        }
        GeometryCommand::Cylinder { position, rotation, radius, height, color, layer } => {
            let material = resources.materials_3d.add(StandardMaterial::from(color));
            cmd_entity.insert((
                Mesh3d(resources.global_geo.cylinder.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(position).with_rotation(rotation).with_scale(Vec3::new(radius, height, radius)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
            ));
        }
        GeometryCommand::Cone { position, rotation, radius, height, color, layer } => {
            let material = resources.materials_3d.add(StandardMaterial::from(color));
            cmd_entity.insert((
                Mesh3d(resources.global_geo.cone.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(position).with_rotation(rotation).with_scale(Vec3::new(radius, height, radius)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
            ));
        }
        GeometryCommand::Torus { position, rotation, radius, tube_radius, color, layer } => {
            let material = resources.materials_3d.add(StandardMaterial::from(color));
            cmd_entity.insert((
                Mesh3d(resources.global_geo.torus.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(position).with_rotation(rotation),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
            ));
        }
        GeometryCommand::Plane { position, rotation, size, color, layer } => {
            let material = resources.materials_3d.add(StandardMaterial::from(color));
            cmd_entity.insert((
                Mesh3d(resources.global_geo.plane.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(position).with_rotation(rotation).with_scale(Vec3::new(size, 1.0, size)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
            ));
        }
        GeometryCommand::Quad { position, rotation, size, color, layer } => {
            let material = resources.materials_3d.add(StandardMaterial::from(color));
            cmd_entity.insert((
                Mesh3d(resources.global_geo.plane.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(position)
                    .with_rotation(rotation)
                    .with_scale(Vec3::new(size.x, 1.0, size.y)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: Some(material) },
            ));
        }
        GeometryCommand::Model { position, rotation, scale, scene, layer } => {
            cmd_entity.insert((
                SceneRoot(scene),
                Transform::from_translation(position).with_rotation(rotation).with_scale(scale),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: None },
            ));
        }
    }
}
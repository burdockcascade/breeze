use std::cell::RefCell;
use bevy::prelude::*;
use bevy::camera::visibility::RenderLayers;
use bevy::ecs::system::SystemParam;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

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

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HashableColor(Color);

impl Eq for HashableColor {}

impl Hash for HashableColor {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Convert to linear RGBA (f32) and hash bits
        let vec = self.0.to_linear().to_f32_array();
        for f in vec {
            state.write_u32(f.to_bits());
        }
    }
}

#[derive(Resource, Default)]
pub struct MaterialCache {
    pub solid_2d: HashMap<HashableColor, Handle<ColorMaterial>>,
    pub solid_3d: HashMap<HashableColor, Handle<StandardMaterial>>,
}

impl MaterialCache {
    pub fn get_2d(&mut self, color: Color, assets: &mut Assets<ColorMaterial>) -> Handle<ColorMaterial> {
        let key = HashableColor(color);
        if let Some(handle) = self.solid_2d.get(&key) {
            return handle.clone();
        }
        let handle = assets.add(ColorMaterial::from(color));
        self.solid_2d.insert(key, handle.clone());
        handle
    }

    pub fn get_3d(&mut self, color: Color, assets: &mut Assets<StandardMaterial>) -> Handle<StandardMaterial> {
        let key = HashableColor(color);
        if let Some(handle) = self.solid_3d.get(&key) {
            return handle.clone();
        }
        let handle = assets.add(StandardMaterial::from(color));
        self.solid_3d.insert(key, handle.clone());
        handle
    }
}

/// Commands to be executed by the renderer.
#[derive(Clone)]
pub enum GeometryCommand {
    // --- UNLIT 2D ---
    Circle { position: Vec2, radius: f32, color: Color, layer: usize },
    Rect { position: Vec2, size: Vec2, color: Color, layer: usize },
    Line { start: Vec2, end: Vec2, thickness: f32, color: Color, layer: usize },
    Ring { position: Vec2, radius: f32, thickness: f32, color: Color, layer: usize },

    // --- LIT 3D & LIT 2D ---
    Cube { position: Vec3, rotation: Quat, size: f32, color: Color, layer: usize },
    Cuboid { position: Vec3, rotation: Quat, size: Vec3, color: Color, layer: usize },
    Sphere { position: Vec3, radius: f32, color: Color, layer: usize },
    Cylinder { position: Vec3, rotation: Quat, radius: f32, height: f32, color: Color, layer: usize },
    Cone { position: Vec3, rotation: Quat, radius: f32, height: f32, color: Color, layer: usize },
    Torus { position: Vec3, rotation: Quat, radius: f32, tube_radius: f32, color: Color, layer: usize },
    Plane { position: Vec3, rotation: Quat, size: f32, color: Color, layer: usize },
    Quad { position: Vec3, rotation: Quat, size: Vec2, color: Color, layer: usize },

    // Imported Models
    Model { position: Vec3, rotation: Quat, scale: Vec3, scene: Handle<Scene>, layer: usize },
}

// =================================================================================
//  FRONTEND: USER API (Unchanged)
// =================================================================================

pub struct Geometry2d<'a> {
    pub queue: &'a RefCell<&'a mut GraphicsQueue>,
    pub layer_id: usize,
}

impl<'a> Geometry2d<'a> {
    pub fn circle(&self, position: Vec2, radius: f32, color: Color) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Geometry(
            GeometryCommand::Circle { position, radius, color, layer: self.layer_id }
        ));
    }
    pub fn rect(&self, position: Vec2, size: Vec2, color: Color) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Geometry(
            GeometryCommand::Rect { position, size, color, layer: self.layer_id }
        ));
    }
    pub fn line(&self, start: Vec2, end: Vec2, thickness: f32, color: Color) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Geometry(
            GeometryCommand::Line { start, end, thickness, color, layer: self.layer_id }
        ));
    }
    pub fn ring(&self, position: Vec2, radius: f32, thickness: f32, color: Color) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Geometry(
            GeometryCommand::Ring { position, radius, thickness, color, layer: self.layer_id }
        ));
    }
}

pub struct Geometry3d<'a> {
    pub queue: &'a RefCell<&'a mut GraphicsQueue>,
    pub layer_id: usize,
}

impl<'a> Geometry3d<'a> {
    pub fn cube(&self, position: Vec3, rotation: Quat, size: f32, color: Color) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Geometry(
            GeometryCommand::Cube { position, rotation, size, color, layer: self.layer_id }
        ));
    }
    pub fn cuboid(&self, position: Vec3, rotation: Quat, size: Vec3, color: Color) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Geometry(
            GeometryCommand::Cuboid { position, rotation, size, color, layer: self.layer_id }
        ));
    }
    pub fn sphere(&self, position: Vec3, radius: f32, color: Color) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Geometry(
            GeometryCommand::Sphere { position, radius, color, layer: self.layer_id }
        ));
    }
    pub fn cylinder(&self, position: Vec3, rotation: Quat, radius: f32, height: f32, color: Color) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Geometry(
            GeometryCommand::Cylinder { position, rotation, radius, height, color, layer: self.layer_id }
        ));
    }
    pub fn cone(&self, position: Vec3, rotation: Quat, radius: f32, height: f32, color: Color) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Geometry(
            GeometryCommand::Cone { position, rotation, radius, height, color, layer: self.layer_id }
        ));
    }
    pub fn torus(&self, position: Vec3, rotation: Quat, radius: f32, tube_radius: f32, color: Color) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Geometry(
            GeometryCommand::Torus { position, rotation, radius, tube_radius, color, layer: self.layer_id }
        ));
    }
    pub fn plane(&self, position: Vec3, rotation: Quat, size: f32, color: Color) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Geometry(
            GeometryCommand::Plane { position, rotation, size, color, layer: self.layer_id }
        ));
    }
    pub fn quad(&self, position: Vec3, rotation: Quat, size: Vec2, color: Color) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Geometry(
            GeometryCommand::Quad { position, rotation, size, color, layer: self.layer_id }
        ));
    }
    pub fn model(&self, position: Vec3, rotation: Quat, scale: Vec3, scene: Handle<Scene>) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Geometry(
            GeometryCommand::Model { position, rotation, scale, scene, layer: self.layer_id }
        ));
    }
}

// =================================================================================
//  RENDERER SYSTEM (IMPROVED)
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
    pub material_cache: ResMut<'w, MaterialCache>,

    pub q_transient: Query<'w, 's, (Entity, &'static mut TransientResources)>,

    // --- FAST PATH QUERIES ---
    pub shapes: ParamSet<'w, 's, (
        // 0: Mesh2d (Fast Path)
        Query<'w, 's, (
            &'static mut Mesh2d,
            &'static mut MeshMaterial2d<ColorMaterial>,
            &'static mut Transform,
            &'static mut Visibility,
            &'static mut RenderLayers
        )>,
        // 1: Mesh3d (Fast Path)
        Query<'w, 's, (
            &'static mut Mesh3d,
            &'static mut MeshMaterial3d<StandardMaterial>,
            &'static mut Transform,
            &'static mut Visibility,
            &'static mut RenderLayers
        )>,
        // 2: SceneRoot (Fast Path for Models)
        Query<'w, 's, (
            &'static mut SceneRoot,
            &'static mut Transform,
            &'static mut Visibility,
            &'static mut RenderLayers
        )>,
    )>,
}

pub fn process_geometry(
    commands: &mut Commands,
    renderer: &mut GeometryRenderer,
    entity_opt: Option<Entity>,
    command: GeometryCommand
) {
    // =====================================================================
    //  FAST PATH: MUTATION
    // =====================================================================
    if let Some(entity) = entity_opt {
        let mut clear_transient = |e: Entity| {
            if let Ok((_, mut res)) = renderer.q_transient.get_mut(e) {
                if let Some(h) = &res.mesh { renderer.meshes.remove(h); }
                res.mesh = None;
            }
        };

        match &command {
            // --- 2D SHAPES ---
            // Use shapes.p0() for Mesh2d
            GeometryCommand::Circle { position, radius, color, layer } => {
                if let Ok((mut mesh, mut mat, mut xform, mut vis, mut layers)) = renderer.shapes.p0().get_mut(entity) {
                    mesh.0 = renderer.global_geo.circle.clone();
                    mat.0 = renderer.material_cache.get_2d(*color, &mut renderer.materials_2d);
                    xform.translation = position.extend(0.0);
                    xform.rotation = Quat::IDENTITY;
                    xform.scale = Vec3::splat(*radius);
                    *vis = Visibility::Visible;
                    *layers = RenderLayers::layer(*layer);
                    clear_transient(entity);
                    return;
                }
            },
            GeometryCommand::Rect { position, size, color, layer } => {
                if let Ok((mut mesh, mut mat, mut xform, mut vis, mut layers)) = renderer.shapes.p0().get_mut(entity) {
                    mesh.0 = renderer.global_geo.rect.clone();
                    mat.0 = renderer.material_cache.get_2d(*color, &mut renderer.materials_2d);
                    xform.translation = position.extend(0.0);
                    xform.rotation = Quat::IDENTITY;
                    xform.scale = size.extend(1.0);
                    *vis = Visibility::Visible;
                    *layers = RenderLayers::layer(*layer);
                    clear_transient(entity);
                    return;
                }
            },

            // --- 3D SHAPES ---
            // Use shapes.p1() for Mesh3d
            GeometryCommand::Cube { position, rotation, size, color, layer } => {
                if let Ok((mut mesh, mut mat, mut xform, mut vis, mut layers)) = renderer.shapes.p1().get_mut(entity) {
                    mesh.0 = renderer.global_geo.cuboid.clone();
                    mat.0 = renderer.material_cache.get_3d(*color, &mut renderer.materials_3d);
                    xform.translation = *position;
                    xform.rotation = *rotation;
                    xform.scale = Vec3::splat(*size);
                    *vis = Visibility::Visible;
                    *layers = RenderLayers::layer(*layer);
                    clear_transient(entity);
                    return;
                }
            },
            // ... (Other 3D shapes use p1()) ...
            GeometryCommand::Cuboid { position, rotation, size, color, layer } => {
                if let Ok((mut mesh, mut mat, mut xform, mut vis, mut layers)) = renderer.shapes.p1().get_mut(entity) {
                    mesh.0 = renderer.global_geo.cuboid.clone();
                    mat.0 = renderer.material_cache.get_3d(*color, &mut renderer.materials_3d);
                    xform.translation = *position;
                    xform.rotation = *rotation;
                    xform.scale = *size;
                    *vis = Visibility::Visible;
                    *layers = RenderLayers::layer(*layer);
                    clear_transient(entity);
                    return;
                }
            },
            GeometryCommand::Sphere { position, radius, color, layer } => {
                if let Ok((mut mesh, mut mat, mut xform, mut vis, mut layers)) = renderer.shapes.p1().get_mut(entity) {
                    mesh.0 = renderer.global_geo.sphere.clone();
                    mat.0 = renderer.material_cache.get_3d(*color, &mut renderer.materials_3d);
                    xform.translation = *position;
                    xform.rotation = Quat::IDENTITY;
                    xform.scale = Vec3::splat(*radius);
                    *vis = Visibility::Visible;
                    *layers = RenderLayers::layer(*layer);
                    clear_transient(entity);
                    return;
                }
            },
            GeometryCommand::Cylinder { position, rotation, radius, height, color, layer } => {
                if let Ok((mut mesh, mut mat, mut xform, mut vis, mut layers)) = renderer.shapes.p1().get_mut(entity) {
                    mesh.0 = renderer.global_geo.cylinder.clone();
                    mat.0 = renderer.material_cache.get_3d(*color, &mut renderer.materials_3d);
                    xform.translation = *position;
                    xform.rotation = *rotation;
                    xform.scale = Vec3::new(*radius, *height, *radius);
                    *vis = Visibility::Visible;
                    *layers = RenderLayers::layer(*layer);
                    clear_transient(entity);
                    return;
                }
            },
            GeometryCommand::Cone { position, rotation, radius, height, color, layer } => {
                if let Ok((mut mesh, mut mat, mut xform, mut vis, mut layers)) = renderer.shapes.p1().get_mut(entity) {
                    mesh.0 = renderer.global_geo.cone.clone();
                    mat.0 = renderer.material_cache.get_3d(*color, &mut renderer.materials_3d);
                    xform.translation = *position;
                    xform.rotation = *rotation;
                    xform.scale = Vec3::new(*radius, *height, *radius);
                    *vis = Visibility::Visible;
                    *layers = RenderLayers::layer(*layer);
                    clear_transient(entity);
                    return;
                }
            },
            GeometryCommand::Torus { position, rotation, radius, tube_radius, color, layer } => {
                if let Ok((mut mesh, mut mat, mut xform, mut vis, mut layers)) = renderer.shapes.p1().get_mut(entity) {
                    mesh.0 = renderer.global_geo.torus.clone();
                    mat.0 = renderer.material_cache.get_3d(*color, &mut renderer.materials_3d);
                    xform.translation = *position;
                    xform.rotation = *rotation;
                    xform.scale = Vec3::splat(*radius);
                    *vis = Visibility::Visible;
                    *layers = RenderLayers::layer(*layer);
                    clear_transient(entity);
                    return;
                }
            },
            GeometryCommand::Plane { position, rotation, size, color, layer } => {
                if let Ok((mut mesh, mut mat, mut xform, mut vis, mut layers)) = renderer.shapes.p1().get_mut(entity) {
                    mesh.0 = renderer.global_geo.plane.clone();
                    mat.0 = renderer.material_cache.get_3d(*color, &mut renderer.materials_3d);
                    xform.translation = *position;
                    xform.rotation = *rotation;
                    xform.scale = Vec3::new(*size, 1.0, *size);
                    *vis = Visibility::Visible;
                    *layers = RenderLayers::layer(*layer);
                    clear_transient(entity);
                    return;
                }
            },
            GeometryCommand::Quad { position, rotation, size, color, layer } => {
                if let Ok((mut mesh, mut mat, mut xform, mut vis, mut layers)) = renderer.shapes.p1().get_mut(entity) {
                    mesh.0 = renderer.global_geo.plane.clone();
                    mat.0 = renderer.material_cache.get_3d(*color, &mut renderer.materials_3d);
                    xform.translation = *position;
                    xform.rotation = *rotation;
                    xform.scale = Vec3::new(size.x, 1.0, size.y);
                    *vis = Visibility::Visible;
                    *layers = RenderLayers::layer(*layer);
                    clear_transient(entity);
                    return;
                }
            },

            // --- MODEL ---
            // Use shapes.p2() for SceneRoot
            GeometryCommand::Model { position, rotation, scale, scene, layer } => {
                if let Ok((mut scene_root, mut xform, mut vis, mut layers)) = renderer.shapes.p2().get_mut(entity) {
                    if scene_root.0 != *scene { scene_root.0 = scene.clone(); }
                    xform.translation = *position;
                    xform.rotation = *rotation;
                    xform.scale = *scale;
                    *vis = Visibility::Visible;
                    *layers = RenderLayers::layer(*layer);
                    clear_transient(entity);
                    return;
                }
            },
            _ => {}
        }
    }

    // =====================================================================
    //  SLOW PATH: REBUILD
    // =====================================================================
    
    // 1. Cleanup logic (Same as before: remove transient meshes, strip components)
    if let Some(entity) = entity_opt {
        if let Ok((_, res)) = renderer.q_transient.get(entity) {
            if let Some(handle) = &res.mesh { renderer.meshes.remove(handle); }
        }
        commands.entity(entity)
            .remove::<Mesh2d>()
            .remove::<MeshMaterial2d<ColorMaterial>>()
            .remove::<Mesh3d>()
            .remove::<MeshMaterial3d<StandardMaterial>>()
            .remove::<SceneRoot>();
    }

    let mut cmd_entity = if let Some(e) = entity_opt { commands.entity(e) } else { commands.spawn(()) };

    // 2. Insert Bundles
    match command {
        // --- 2D ---
        GeometryCommand::Circle { position, radius, color, layer } => {
            let material = renderer.material_cache.get_2d(color, &mut renderer.materials_2d);
            cmd_entity.insert((
                Mesh2d(renderer.global_geo.circle.clone()),
                MeshMaterial2d(material),
                Transform::from_translation(position.extend(0.0)).with_scale(Vec3::splat(radius)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: None },
            ));
        }
        GeometryCommand::Rect { position, size, color, layer } => {
            let material = renderer.material_cache.get_2d(color, &mut renderer.materials_2d);
            cmd_entity.insert((
                Mesh2d(renderer.global_geo.rect.clone()),
                MeshMaterial2d(material),
                Transform::from_translation(position.extend(0.0)).with_scale(size.extend(1.0)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: None },
            ));
        }
        GeometryCommand::Line { start, end, thickness, color, layer } => {
            let center = (start + end) / 2.0;
            let length = start.distance(end);
            let angle = (end.y - start.y).atan2(end.x - start.x);
            let material = renderer.material_cache.get_2d(color, &mut renderer.materials_2d);
            cmd_entity.insert((
                Mesh2d(renderer.global_geo.rect.clone()),
                MeshMaterial2d(material),
                Transform::from_translation(center.extend(0.0))
                    .with_rotation(Quat::from_rotation_z(angle))
                    .with_scale(Vec3::new(length, thickness, 1.0)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: None },
            ));
        }
        GeometryCommand::Ring { position, radius, thickness, color, layer } => {
            let inner = radius - thickness / 2.0;
            let outer = radius + thickness / 2.0;
            // Unique Mesh -> Create it
            let mesh_handle = renderer.meshes.add(Annulus::new(inner, outer));
            let material = renderer.material_cache.get_2d(color, &mut renderer.materials_2d);
            cmd_entity.insert((
                Mesh2d(mesh_handle.clone()),
                MeshMaterial2d(material),
                Transform::from_translation(position.extend(0.0)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                // Track mesh so we can delete it next frame
                TransientResources { mesh: Some(mesh_handle), material_2d: None, material_3d: None },
            ));
        }

        // --- 3D ---
        GeometryCommand::Cube { position, rotation, size, color, layer } => {
            let material = renderer.material_cache.get_3d(color, &mut renderer.materials_3d);
            cmd_entity.insert((
                Mesh3d(renderer.global_geo.cuboid.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(position).with_rotation(rotation).with_scale(Vec3::splat(size)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: None },
            ));
        }
        GeometryCommand::Cuboid { position, rotation, size, color, layer } => {
            let material = renderer.material_cache.get_3d(color, &mut renderer.materials_3d);
            cmd_entity.insert((
                Mesh3d(renderer.global_geo.cuboid.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(position).with_rotation(rotation).with_scale(size),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: None },
            ));
        }
        GeometryCommand::Sphere { position, radius, color, layer } => {
            let material = renderer.material_cache.get_3d(color, &mut renderer.materials_3d);
            cmd_entity.insert((
                Mesh3d(renderer.global_geo.sphere.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(position).with_scale(Vec3::splat(radius)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: None },
            ));
        }
        GeometryCommand::Cylinder { position, rotation, radius, height, color, layer } => {
            let material = renderer.material_cache.get_3d(color, &mut renderer.materials_3d);
            cmd_entity.insert((
                Mesh3d(renderer.global_geo.cylinder.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(position).with_rotation(rotation).with_scale(Vec3::new(radius, height, radius)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: None },
            ));
        }
        GeometryCommand::Cone { position, rotation, radius, height, color, layer } => {
            let material = renderer.material_cache.get_3d(color, &mut renderer.materials_3d);
            cmd_entity.insert((
                Mesh3d(renderer.global_geo.cone.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(position).with_rotation(rotation).with_scale(Vec3::new(radius, height, radius)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: None },
            ));
        }
        GeometryCommand::Torus { position, rotation, radius: _, tube_radius: _, color, layer } => {
            let material = renderer.material_cache.get_3d(color, &mut renderer.materials_3d);
            cmd_entity.insert((
                Mesh3d(renderer.global_geo.torus.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(position).with_rotation(rotation),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: None },
            ));
        }
        GeometryCommand::Plane { position, rotation, size, color, layer } => {
            let material = renderer.material_cache.get_3d(color, &mut renderer.materials_3d);
            cmd_entity.insert((
                Mesh3d(renderer.global_geo.plane.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(position).with_rotation(rotation).with_scale(Vec3::new(size, 1.0, size)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: None },
            ));
        }
        GeometryCommand::Quad { position, rotation, size, color, layer } => {
            let material = renderer.material_cache.get_3d(color, &mut renderer.materials_3d);
            cmd_entity.insert((
                Mesh3d(renderer.global_geo.plane.clone()),
                MeshMaterial3d(material),
                Transform::from_translation(position)
                    .with_rotation(rotation)
                    .with_scale(Vec3::new(size.x, 1.0, size.y)),
                RenderLayers::layer(layer),
                Visibility::Visible,
                TransientResources { mesh: None, material_2d: None, material_3d: None },
            ));
        }

        // --- MODEL ---
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
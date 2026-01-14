use bevy::prelude::*;
use bevy::camera::visibility::RenderLayers;

/// Resource to store the Unit meshes (Circle, Rect) so we don't recreate them every frame.
#[derive(Resource)]
pub struct GlobalShapeResources {
    pub circle: Handle<Mesh>,
    pub rect: Handle<Mesh>,
}

impl FromWorld for GlobalShapeResources {
    fn from_world(world: &mut World) -> Self {
        let mut meshes = world.resource_mut::<Assets<Mesh>>();
        // Create a unit circle (radius 1.0)
        let circle = meshes.add(Circle::new(1.0));
        // Create a unit rectangle (1.0 x 1.0)
        let rect = meshes.add(Rectangle::new(1.0, 1.0));
        Self { circle, rect }
    }
}

/// Commands to be executed by the renderer
#[derive(Clone, Copy)]
pub enum ShapeCommand {
    Circle { x: f32, y: f32, radius: f32, color: Color, layer: usize },
    Rect { x: f32, y: f32, w: f32, h: f32, color: Color, layer: usize },
    Line { x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color, layer: usize },
    Ring { x: f32, y: f32, radius: f32, thickness: f32, color: Color, layer: usize },
}

/// Resource to queue up shape commands for the current frame
#[derive(Resource, Default)]
pub struct ShapeQueue(pub Vec<ShapeCommand>);

/// Component to tag shapes that should be despawned at the end of the frame
/// Also holds handles to assets that need to be manually cleaned up (dynamic meshes/materials)
#[derive(Component)]
pub struct TransientResources {
    pub mesh: Option<Handle<Mesh>>,
    pub material: Option<Handle<ColorMaterial>>,
}

pub struct ShapeContext<'a> {
    pub queue: &'a mut ShapeQueue,
    pub layer_id: usize,
}

impl<'a> ShapeContext<'a> {
    pub fn new(queue: &'a mut ShapeQueue, layer_id: usize) -> Self {
        Self { queue, layer_id }
    }

    /// Draw a circle at (x, y) with given radius
    pub fn circle(&mut self, x: f32, y: f32, radius: f32, color: Color) {
        self.queue.0.push(ShapeCommand::Circle { x, y, radius, color, layer: self.layer_id });
    }

    /// Draw a rectangle at (x, y) with width w and height h
    pub fn rect(&mut self, x: f32, y: f32, w: f32, h: f32, color: Color) {
        self.queue.0.push(ShapeCommand::Rect { x, y, w, h, color, layer: self.layer_id });
    }

    /// Draw a line from (x1, y1) to (x2, y2) with given thickness
    pub fn line(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, thickness: f32, color: Color) {
        self.queue.0.push(ShapeCommand::Line { x1, y1, x2, y2, thickness, color, layer: self.layer_id });
    }

    /// Draw a ring at (x, y) with given radius and thickness
    pub fn ring(&mut self, x: f32, y: f32, radius: f32, thickness: f32, color: Color) {
        self.queue.0.push(ShapeCommand::Ring { x, y, radius, thickness, color, layer: self.layer_id });
    }

}

pub fn render_shapes(mut commands: Commands, mut queue: ResMut<ShapeQueue>, global_shapes: Res<GlobalShapeResources>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, q_transient: Query<(Entity, &TransientResources)>) {

    // Clean up previous frame's transient shapes
    for (entity, resources) in q_transient.iter() {
        if let Some(handle) = &resources.mesh {
            meshes.remove(handle);
        }
        if let Some(handle) = &resources.material {
            materials.remove(handle);
        }
        commands.entity(entity).despawn();
    }

    // Process the queue
    for command in queue.0.drain(..) {
        match command {
            ShapeCommand::Circle { x, y, radius, color, layer } => {
                let material = materials.add(ColorMaterial::from(color));
                commands.spawn((
                    Mesh2d(global_shapes.circle.clone()),
                    MeshMaterial2d(material.clone()),
                    Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(radius)),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: None, material: Some(material) },
                ));
            }
            ShapeCommand::Rect { x, y, w, h, color, layer } => {
                let material = materials.add(ColorMaterial::from(color));
                commands.spawn((
                    Mesh2d(global_shapes.rect.clone()),
                    MeshMaterial2d(material.clone()),
                    Transform::from_xyz(x, y, 0.0).with_scale(Vec3::new(w, h, 1.0)),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: None, material: Some(material) },
                ));
            }
            ShapeCommand::Line { x1, y1, x2, y2, thickness, color, layer } => {
                let p1 = Vec2::new(x1, y1);
                let p2 = Vec2::new(x2, y2);
                let center = (p1 + p2) / 2.0;
                let length = p1.distance(p2);
                let angle = (p2.y - p1.y).atan2(p2.x - p1.x);

                let material = materials.add(ColorMaterial::from(color));
                commands.spawn((
                    Mesh2d(global_shapes.rect.clone()),
                    MeshMaterial2d(material.clone()),
                    Transform::from_xyz(center.x, center.y, 0.0)
                        .with_rotation(Quat::from_rotation_z(angle))
                        .with_scale(Vec3::new(length, thickness, 1.0)),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: None, material: Some(material) },
                ));
            }
            ShapeCommand::Ring { x, y, radius, thickness, color, layer } => {
                let inner = radius - thickness / 2.0;
                let outer = radius + thickness / 2.0;
                let mesh_handle = meshes.add(Annulus::new(inner, outer));
                let mat_handle = materials.add(ColorMaterial::from(color));
                
                commands.spawn((
                    Mesh2d(mesh_handle.clone()),
                    MeshMaterial2d(mat_handle.clone()),
                    Transform::from_xyz(x, y, 0.0),
                    RenderLayers::layer(layer),
                    TransientResources { mesh: Some(mesh_handle), material: Some(mat_handle) },
                ));
            }
        }
    }
}
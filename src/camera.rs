use bevy::camera::visibility::RenderLayers;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraMode {
    None,
    Camera2d {
        position: Vec2,
        scale: f32
    },
    // New 3D Mode
    Camera3d {
        position: Vec3,
        target: Vec3,
    }
}

impl Default for CameraMode {
    fn default() -> Self {
        CameraMode::Camera2d {
            position: Vec2::ZERO,
            scale: 1.0,
        }
    }
}

#[derive(Resource, Default)]
pub struct CameraQueue(pub Vec<(usize, CameraMode)>);

#[derive(QueryData)]
#[query_data(mutable)]
pub struct CameraItem {
    pub entity: Entity,
    pub layers: &'static RenderLayers,
    pub transform: Option<&'static mut Transform>,
    pub projection: Option<&'static mut Projection>,
    pub cam2d: Option<&'static Camera2d>,
    pub cam3d: Option<&'static Camera3d>,
}

pub fn manage_cameras(mut commands: Commands, mut queue: ResMut<CameraQueue>, mut query: Query<CameraItem, With<Camera>>) {
    for (layer, mode) in queue.0.drain(..) {
        let target_layer = RenderLayers::layer(layer);
        let mut found = false;

        // Update existing cameras
        for mut item in query.iter_mut() {
            if item.layers == &target_layer {
                found = true;

                match mode {
                    CameraMode::None => {
                        commands.entity(item.entity).despawn();
                    }
                    CameraMode::Camera2d { position, scale } => {
                        // Switch to 2D if needed
                        if item.cam2d.is_none() {
                            commands.entity(item.entity)
                                .remove::<Camera3d>()
                                .insert(Camera2d::default());
                        }

                        // Update Position
                        if let Some(ref mut t) = item.transform {
                            t.translation = position.extend(0.0);
                            t.look_to(Vec3::NEG_Z, Vec3::Y); // Reset rotation for 2D
                        }

                        // Update Projection
                        if let Some(ref mut proj) = item.projection {
                            if let Projection::Orthographic(ref mut ortho) = **proj {
                                ortho.scale = scale;
                            } else {
                                **proj = Projection::Orthographic(OrthographicProjection {
                                    scale,
                                    ..OrthographicProjection::default_2d()
                                });
                            }
                        }
                    }
                    CameraMode::Camera3d { position, target } => {
                        // Switch to 3D if needed
                        if item.cam3d.is_none() {
                            commands.entity(item.entity)
                                .remove::<Camera2d>()
                                .insert(Camera3d::default());
                        }

                        // Update Position & Rotation safely
                        if let Some(ref mut t) = item.transform {
                            t.translation = position;

                            // Only look_at if we aren't standing exactly on the target
                            if position != target {
                                t.look_at(target, Vec3::Y);
                            }
                        }

                        // Update Projection to Perspective
                        if let Some(ref mut proj) = item.projection {
                            if !matches!(**proj, Projection::Perspective(_)) {
                                **proj = Projection::Perspective(PerspectiveProjection::default());
                            }
                        }
                    }
                }
            }
        }

        // Spawn new camera if not found
        if !found && mode != CameraMode::None {
            let camera_base = Camera {
                order: layer as isize,
                clear_color: if layer == 0 { ClearColorConfig::Default } else { ClearColorConfig::None },
                ..default()
            };

            match mode {
                CameraMode::Camera2d { position, scale } => {
                    commands.spawn((
                        Camera2d::default(),
                        camera_base,
                        target_layer,
                        Projection::Orthographic(OrthographicProjection {
                            scale,
                            ..OrthographicProjection::default_2d()
                        }),
                        Transform::from_translation(position.extend(0.0)),
                    ));
                },
                CameraMode::Camera3d { position, target } => {
                    commands.spawn((
                        Camera3d::default(),
                        camera_base,
                        target_layer,
                        Projection::Perspective(PerspectiveProjection::default()),
                        Transform::from_translation(position).looking_at(target, Vec3::Y),
                    ));
                },
                CameraMode::None => { }
            }
        }
    }
}
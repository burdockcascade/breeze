// src/graphics/commands.rs
use bevy::prelude::*;
use crate::graphics::geometry::GeometryCommand;
use crate::graphics::sprite::SpriteCommand;
use crate::graphics::text::TextCommand;
use crate::graphics::lights::LightCommand;

#[derive(Resource, Default)]    
pub struct GraphicsQueue(pub Vec<GraphicsCommand>);

#[derive(Clone)]
pub enum GraphicsCommand {
    Geometry(GeometryCommand),
    Sprite(SpriteCommand),
    Text(TextCommand),
    Light(LightCommand),
}
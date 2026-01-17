use bevy::prelude::*;
use bevy::camera::visibility::RenderLayers;
use bevy::ecs::system::SystemParam;
use std::cell::RefCell;

// Import the Unified Types
use crate::graphics::commands::{GraphicsCommand, GraphicsQueue};

// --- 1. COMMAND DATA ---
#[derive(Clone)]
pub struct TextCommand {
    pub text: String,
    pub font: Handle<Font>,
    pub position: Vec2,
    pub size: f32,
    pub color: Color,
    pub layer: usize,
}

// --- 2. MARKER COMPONENT (For Cleanup) ---
#[derive(Component)]
pub struct ImmediateText;

// --- 3. CONTEXT (Frontend) ---
pub struct TextContext<'a> {
    pub queue: &'a RefCell<&'a mut GraphicsQueue>,
    pub layer_id: usize,
}

impl<'a> TextContext<'a> {

    pub fn draw(&self, text: impl Into<String>, position: Vec2) {
        self.draw_pro(&Handle::default(), text, position, 16.0, Color::BLACK);
    }

    pub fn draw_ext(&self, text: impl Into<String>, position: Vec2, size: f32, color: Color) {
        self.draw_pro(&Handle::default(), text, position, size, color);
    }

    /// Draw text at the specified position.
    pub fn draw_pro(&self, font: &Handle<Font>, text: impl Into<String>, position: Vec2, size: f32, color: Color) {
        self.queue.borrow_mut().0.push(GraphicsCommand::Text(TextCommand {
            text: text.into(),
            font: font.clone(),
            position,
            size,
            color,
            layer: self.layer_id,
        }));
    }
}

// --- 4. RENDERER RESOURCES (Backend) ---
#[derive(SystemParam)]
pub struct TextRenderer<'w, 's> {
    // Query to find text entities from the previous frame for cleanup
    pub q_text: Query<'w, 's, Entity, With<ImmediateText>>,
}

// --- 5. SPAWN HELPER (Called by UnifiedRenderer) ---
pub fn process_text(
    commands: &mut Commands,
    entity_opt: Option<Entity>,
    cmd: TextCommand
) {
    let mut e = if let Some(entity) = entity_opt {
        commands.entity(entity)
    } else {
        commands.spawn((
            ImmediateText,
        ))
    };

    e.insert((
        Text2d::new(cmd.text),
        TextFont {
            font: cmd.font,
            font_size: cmd.size,
            ..default()
        },
        TextColor(cmd.color),
        Transform::from_translation(cmd.position.extend(0.0)),
        RenderLayers::layer(cmd.layer),
        Visibility::Visible,
    ));
}
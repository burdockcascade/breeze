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
    // We add components here to allow Mutable access (Fast Path)
    pub q_text: Query<'w, 's, (
        Entity,
        &'static mut Text2d,
        &'static mut TextFont,
        &'static mut TextColor,
        &'static mut Transform,
        &'static mut Visibility,
        &'static mut RenderLayers
    ), With<ImmediateText>>,
}

// --- 5. SPAWN HELPER (Called by UnifiedRenderer) ---
pub fn process_text(
    commands: &mut Commands,
    renderer: &mut TextRenderer, // CHANGED: Now takes mutable renderer
    entity_opt: Option<Entity>,
    cmd: TextCommand
) {
    // 1. FAST PATH: Update existing entity
    if let Some(entity) = entity_opt {
        if let Ok((_, mut txt, mut font, mut color, mut xform, mut vis, mut layers)) = renderer.q_text.get_mut(entity) {
            txt.0 = cmd.text;
            font.font = cmd.font;
            font.font_size = cmd.size;
            color.0 = cmd.color;
            xform.translation = cmd.position.extend(0.0);
            *vis = Visibility::Visible;
            *layers = RenderLayers::layer(cmd.layer);
            return;
        }
    }

    // 2. SLOW PATH: Spawn new
    commands.spawn((
        ImmediateText,
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
use bevy::ecs::query::QueryData;
use bevy::prelude::*;
use bevy::sprite::Anchor;

// Command struct for queuing text rendering
#[derive(Clone)]
pub struct TextCommand {
    pub text: String,
    pub position: Vec2,
    pub size: f32,
    pub color: Color,
}

// Resource to hold queued text commands
#[derive(Resource, Default)]
pub struct TextQueue(pub Vec<TextCommand>);

// Marker component for immediate text entities
#[derive(Component)]
pub struct ImmediateText;

pub struct TextContext<'a, 'w> {
    pub text_queue: &'a mut ResMut<'w, TextQueue>,
}

impl<'a, 'w> TextContext<'a, 'w> {

    pub fn new(text_queue: &'a mut ResMut<'w, TextQueue>) -> Self {
        Self { text_queue }
    }

    pub fn draw(&mut self, text: &str, x: f32, y: f32) {
        self.text_queue.0.push(TextCommand {
            text: text.to_string(),
            position: Vec2::new(x, y),
            size: 20.0,
            color: Color::WHITE,
        });
    }

    /// Draw text with size and color
    pub fn draw_ext(&mut self, text: &str, x: f32, y: f32, size: f32, color: Color) {
        self.text_queue.0.push(TextCommand {
            text: text.to_string(),
            position: Vec2::new(x, y),
            size,
            color,
        });
    }

}

// QueryData struct for mutable access to text item components
#[derive(QueryData)]
#[query_data(mutable)]
pub struct TextItem {
    pub entity: Entity,
    pub text: &'static mut Text2d,
    pub transform: &'static mut Transform,
    pub font: &'static mut TextFont,
    pub color: &'static mut TextColor,
    pub layout: &'static mut TextLayout,
    pub anchor: &'static mut Anchor,
    pub visibility: &'static mut Visibility,
}

pub fn render_text(mut commands: Commands, mut queue: ResMut<TextQueue>, mut query: Query<TextItem, With<ImmediateText>>) {
    let mut drawn_count = 0;

    // Update existing entities
    for ((i, command), mut text_item) in queue.0.iter().enumerate().zip(query.iter_mut()) {

        // Update content
        text_item.text.0 = command.text.clone();

        // Update position
        let z_depth = 1.0 + (i as f32 * 0.0001); // Slightly offset z to avoid z-fighting
        text_item.transform.translation = Vec3::new(command.position.x, command.position.y, z_depth);

        // Update style
        text_item.font.font_size = command.size;
        text_item.color.0 = command.color;

        *text_item.layout = TextLayout::default();
        *text_item.anchor = Anchor::default();

        *text_item.visibility = Visibility::Visible;
        drawn_count += 1;
    }

    // If there are more commands than existing entities, spawn new ones
    if queue.0.len() > drawn_count {
        for command in queue.0.iter().skip(drawn_count) {
            commands.spawn((
                Text2d::new(command.text.clone()),
                Transform::from_xyz(command.position.x, command.position.y, 1.0),
                TextFont {
                    font_size: command.size,
                    ..default()
                },
                TextColor(command.color),
                TextLayout::default(),
                Anchor::default(),
                Visibility::Visible,
                ImmediateText,
            ));
        }
    }

    // Hide unused entities
    for mut text_item in query.iter_mut().skip(drawn_count) {
        *text_item.visibility = Visibility::Hidden;
    }

    // Clear the queue
    queue.0.clear();
}
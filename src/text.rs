use bevy::camera::visibility::RenderLayers;
use bevy::ecs::query::QueryData;
use bevy::prelude::*;

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
pub struct TextQueue(pub Vec<Vec<TextCommand>>);

// Marker component for immediate text entities
#[derive(Component)]
pub struct ImmediateText;

pub struct TextContext<'a> {
    pub queue: &'a mut TextQueue,
    pub layer_id: usize,
}

impl<'a> TextContext<'a> {

    pub fn new(queue: &'a mut TextQueue, layer_id: usize) -> Self {
        Self {
            queue,
            layer_id,
        }
    }

    fn get_queue(&mut self) -> &mut Vec<TextCommand> {
        if self.layer_id >= self.queue.0.len() {
            self.queue.0.resize_with(self.layer_id + 1, Vec::new);
        }
        &mut self.queue.0[self.layer_id]
    }

    pub fn draw(&mut self, text: &str, x: f32, y: f32) {
       self.draw_ext(text, x, y, 16.0, Color::BLACK);
    }

    /// Draw text with size and color
    pub fn draw_ext(&mut self, text: &str, x: f32, y: f32, size: f32, color: Color) {
        self.get_queue().push(TextCommand {
            text: text.to_owned(),
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
    pub visibility: &'static mut Visibility,
    pub layers: Option<&'static mut RenderLayers>,
}

pub fn render_text(mut commands: Commands, mut queue: ResMut<TextQueue>, mut query: Query<TextItem, With<ImmediateText>>) {

    // Flatten
    let mut flat_commands = Vec::new();
    for (layer_id, cmds) in queue.0.iter().enumerate() {
        for (i, cmd) in cmds.iter().enumerate() {
            flat_commands.push((layer_id, i, cmd));
        }
    }

    let mut drawn_count = 0;

    // Recycle
    for (mut item, (layer_id, index, cmd)) in query.iter_mut().zip(flat_commands.iter()) {

        if item.text.0 != cmd.text {
            item.text.0 = cmd.text.clone();
        }

        // Calculate z-index based on layer and index to ensure proper layering
        let z = (*layer_id as f32 * 100.0) + (*index as f32 * 0.00001);

        // Update transform only if changed
        if item.transform.translation != Vec3::new(cmd.position.x, cmd.position.y, z) {
            item.transform.translation = Vec3::new(cmd.position.x, cmd.position.y, z);
        }

        // Update font size only if changed
        if item.font.font_size != cmd.size {
            item.font.font_size = cmd.size;
        }

        // Update color only if changed
        if item.color.0 != cmd.color {
            item.color.0 = cmd.color;
        }

        // Update visibility
        if *item.visibility != Visibility::Visible {
            *item.visibility = Visibility::Visible;
        }

        let target_layer = RenderLayers::layer(*layer_id);
        if let Some(ref mut l) = item.layers {
            if **l != target_layer { **l = target_layer; }
        } else {
            commands.entity(item.entity).insert(target_layer);
        }

        drawn_count += 1;
    }

    // Spawn
    if flat_commands.len() > drawn_count {
        for (layer_id, _, cmd) in flat_commands.iter().skip(drawn_count) {
            commands.spawn((
                Text2d::new(cmd.text.clone()),
                Transform::from_xyz(cmd.position.x, cmd.position.y, 1.0),
                TextFont { font_size: cmd.size, ..default() },
                TextColor(cmd.color),
                Visibility::Visible,
                ImmediateText,
                RenderLayers::layer(*layer_id),
            ));
        }
    }

    // Hide
    for mut item in query.iter_mut().skip(drawn_count) {
        if *item.visibility != Visibility::Hidden {
            *item.visibility = Visibility::Hidden;
        }
    }

    // Cleanup
    for list in queue.0.iter_mut() {
        list.clear();
    }
}
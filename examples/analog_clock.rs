use breeze::prelude::*;
use std::f32::consts::PI;
use std::time::{SystemTime, UNIX_EPOCH};

struct AnalogClock;

impl Game for AnalogClock {
    fn draw(&mut self, ctx: &mut DrawContext) {
        // Clear background with a dark color
        ctx.clear_background(Color::from(DARK_SLATE_GRAY));

        ctx.with_layer(0, |world| {
            // Setup default 2D camera
            world.set_camera(CameraMode::default());

            // --- Time Calculation ---
            // Get system time (UTC)
            // Note: For local time with timezones, you would typically use the 'chrono' crate.
            // For this example, we use std::time which gives us UTC.
            let now = SystemTime::now();
            let since_epoch = now.duration_since(UNIX_EPOCH).unwrap_or_default();

            let total_seconds = since_epoch.as_secs();
            let millis = since_epoch.subsec_millis();

            // Calculate scalar values for hours, minutes, seconds
            // We add fractional parts for smooth movement
            let raw_hours = (total_seconds / 3600) % 12;
            let raw_minutes = (total_seconds / 60) % 60;
            let raw_seconds = total_seconds % 60;

            let smooth_seconds = raw_seconds as f32 + (millis as f32 / 1000.0);
            let smooth_minutes = raw_minutes as f32 + (smooth_seconds / 60.0);
            let smooth_hours = raw_hours as f32 + (smooth_minutes / 60.0);

            // --- Drawing the Clock Face ---
            // Draw the outer ring
            world.draw2d.ring(vec2(0.0, 0.0), 200.0, 10.0, Color::from(WHITE));

            // Draw hour markers
            for i in 0..12 {
                // 12 markers, spaced by PI/6 radians (30 degrees)
                let angle = i as f32 * (PI / 6.0);
                let inner_radius = 180.0;
                let outer_radius = 200.0;

                let x1 = angle.cos() * inner_radius;
                let y1 = angle.sin() * inner_radius;
                let x2 = angle.cos() * outer_radius;
                let y2 = angle.sin() * outer_radius;

                world.draw2d.line(vec2(x1, y1), vec2(x2, y2), 4.0, Color::from(BLACK));
            }

            // --- Drawing the Hands ---
            // In Bevy/Breeze coordinate system:
            // 0 radians = Right (3 o'clock)
            // PI/2 radians = Up (12 o'clock)
            // Hands move Clockwise, so we subtract from PI/2

            // 1. Hour Hand
            // (12 hours = 2*PI radians)
            let hour_angle = (PI / 2.0) - (smooth_hours * (2.0 * PI) / 12.0);
            draw_hand(world, hour_angle, 100.0, 8.0, Color::from(WHITE));

            // 2. Minute Hand
            // (60 minutes = 2*PI radians)
            let minute_angle = (PI / 2.0) - (smooth_minutes * (2.0 * PI) / 60.0);
            draw_hand(world, minute_angle, 150.0, 5.0, Color::from(WHITE));

            // 3. Second Hand
            // (60 seconds = 2*PI radians)
            let second_angle = (PI / 2.0) - (smooth_seconds * (2.0 * PI) / 60.0);
            draw_hand(world, second_angle, 170.0, 2.0, Color::from(RED));

            // Center cap
            world.draw2d.circle(vec2(0.0, 0.0), 8.0, Color::from(RED));

            // Draw digital time text for reference
            let time_str = format!("{:02}:{:02}:{:02} UTC", raw_hours, raw_minutes, raw_seconds);
            world.text.draw_ext(&time_str, vec2(0.0, -250.0), 30.0, Color::from(WHITE));
        });
    }
}

// Helper function to draw a clock hand from the center
fn draw_hand(world: &mut LayerContext, angle: f32, length: f32, thickness: f32, color: Color) {
    let x = angle.cos() * length;
    let y = angle.sin() * length;
    // Draw line from center (0,0) to calculated tip
    world.draw2d.line(vec2(0.0, 0.0), vec2(x, y), thickness, color);
}

fn main() {
    let config = AppConfig {
        title: "Breeze Analog Clock".to_string(),
        width: 600,
        height: 600,
    };

    run(config, AnalogClock);
}
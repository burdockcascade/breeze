use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct FpsResource {
    pub show_value: f32, // The value we display (updates once per sec)

    // Internal counters
    current_frames: u32,
    timer: f32,
}

/// System running in the background to calculate FPS
pub fn monitor_fps(time: Res<Time>, mut fps: ResMut<FpsResource>) {
    fps.current_frames += 1;
    fps.timer += time.delta_secs();

    if fps.timer >= 1.0 {
        // Calculate average FPS over the last second
        fps.show_value = fps.current_frames as f32 / fps.timer;

        // Reset
        fps.current_frames = 0;
        fps.timer = 0.0;
    }
}
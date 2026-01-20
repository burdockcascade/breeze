use breeze::prelude::*;

struct SystemInfoScene;

impl Scene for SystemInfoScene {
    fn init(&mut self, ctx: &mut Context) {
        // --- System Information ---
        println!("=== SYSTEM INFO ===");
        println!("OS: {} ({})", ctx.system.os(), ctx.system.arch());
        println!("Family: {}", ctx.system.family());
        println!("Cores: {}", ctx.system.cores());

        // --- Graphics Information ---
        // Note: These might be "Unknown" if running in a way where Bevy hasn't fully initialized rendering yet
        println!("\n=== GRAPHICS INFO ===");
        println!("GPU: {}", ctx.system.gpu_name());
        println!("Backend: {}", ctx.system.backend());

        // --- Monitor Information ---
        println!("\n=== MONITORS ===");
        let monitors = ctx.system.monitors();
        if monitors.is_empty() {
            println!("No monitors detected (or running in headless mode).");
        } else {
            for (i, m) in monitors.iter().enumerate() {
                println!(
                    "Monitor #{}: '{}' - {}x{} @ {:.2}Hz (Scale: {:.2})",
                    i, m.name, m.width, m.height, m.refresh_rate, m.scale_factor
                );
            }
        }
    }

    fn update(&mut self, ctx: &mut Context) -> SceneTransition {
        // Just print the frame count every 60 frames to show it updating
        if ctx.system.frame_count() % 60 == 0 {
            println!("Frames Rendered: {}", ctx.system.frame_count());
        }
        SceneTransition::None
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        ctx.clear_background(Color::BLACK);
    }
}

fn main() {
    Breeze::new()
        .title("Hello System")
        .enable_logging(true)
        .run(SystemInfoScene);
}
use breeze::prelude::*;
use rand::Rng; // Add "rand" to dev-dependencies if needed, or use simple math

// --- Components ---

struct Bunny {
    position: Vec2,
    velocity: Vec2,
    color: Color,
}

// --- Game State ---

struct BunnyMarkGame {
    bunnies: Vec<Bunny>,
    texture: ImageAsset,
    paused: bool,
}

impl BunnyMarkGame {
    fn new() -> Self {
        Self {
            bunnies: Vec::new(),
            texture: Default::default(),
            paused: false,
        }
    }

    fn add_bunnies(&mut self, count: usize) {
        let mut rng = rand::rng();

        for _ in 0..count {
            self.bunnies.push(Bunny {
                position: Vec2::ZERO,
                velocity: Vec2::new(
                    rng.random_range(-250.0..250.0),
                    rng.random_range(200.0..800.0),
                ),
                color: Color::srgb(
                    rng.random_range(0.5..1.0),
                    rng.random_range(0.5..1.0),
                    rng.random_range(0.5..1.0),
                ),
            });
        }
    }
}

// --- Game Implementation ---

impl Scene for BunnyMarkGame {
    fn init(&mut self, ctx: &mut Context) {
        self.texture = ctx.load_image("bunny.png");
    }

    fn update(&mut self, ctx: &mut Context) -> SceneTransition {
        // Input: Space to add 1000 bunnies
        if ctx.input.key_down(KeyCode::Space) {
            self.add_bunnies(1000);
        }

        // Input: P to toggle Pause
        if ctx.input.key_down(KeyCode::KeyP) {
            self.paused = !self.paused;
            println!("Paused: {}", self.paused);
        }

        // Input: R to Reset
        if ctx.input.key_down(KeyCode::KeyR) {
            self.bunnies.clear();
            println!("Reset Bunnies");
        }

        // Update Physics (only if not paused)
        if !self.paused {
            let dt = ctx.time.delta_secs();
            let gravity = -2500.0;
            let bounds_x = 400.0; // Half screen width approx
            let bounds_y = 300.0; // Half screen height approx
            let floor_y = -bounds_y + 20.0;

            for bunny in &mut self.bunnies {
                // Apply Gravity
                bunny.velocity.y += gravity * dt;

                // Move
                bunny.position += bunny.velocity * dt;

                // Screen Bounce (Simple)
                if bunny.position.x < -bounds_x {
                    bunny.velocity.x = bunny.velocity.x.abs();
                    bunny.position.x = -bounds_x;
                } else if bunny.position.x > bounds_x {
                    bunny.velocity.x = -bunny.velocity.x.abs();
                    bunny.position.x = bounds_x;
                }

                if bunny.position.y < floor_y {
                    bunny.velocity.y = bunny.velocity.y.abs() * 0.85; // Bounce with damping
                    bunny.position.y = floor_y;

                    // Randomize X velocity slightly on bounce
                    if rand::rng().random_bool(0.3) {
                        bunny.velocity.y += rand::rng().random_range(0.0..200.0);
                    }
                } else if bunny.position.y > bounds_y + 500.0 {
                    // Reset if they fly too high off screen
                    bunny.position.y = bounds_y;
                    bunny.velocity.y = 0.0;
                }
            }
        }
        
        SceneTransition::None
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        // Clear background
        ctx.clear_background(Color::WHITE);

        // Draw Bunnies
        ctx.with_layer(0, |layer| {
            layer.set_camera(CameraMode::default());
            for bunny in &self.bunnies {
                layer.sprites.draw_ext(
                    &self.texture,
                    bunny.position.x,
                    bunny.position.y,
                    1.0, // Scale
                    bunny.color
                );
            }
        });

        // Draw UI
        ctx.with_layer(1, |layer| {
            layer.set_camera(CameraMode::default());
            layer.draw_fps(vec2(300.0, 250.0), Color::BLACK);
            let info = format!("Bunnies: {}\n(Space) Add 1000\n(P) Pause\n(R) Reset",
                               self.bunnies.len()
            );
            layer.text.draw_ext(&info, vec2(-0.0, 100.0), 20.0, Color::BLACK);
        });
    }
}

fn main() {
    Breeze::default()
        .title("Bunnymark Example")
        .resolution(800, 600)
        .run(BunnyMarkGame::new());
}
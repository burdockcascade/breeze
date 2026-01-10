# Breeze
An opinionated, immediate-mode game framework for Rust.

## What is it?
Breeze is a lightweight wrapper around the Bevy Engine designed for creative coding, prototyping, and game jams.

It completely abstracts away the Entity Component System (ECS). Instead of managing Entities, Queries, and Systems, you get a simple, familiar update() and draw() loop. You write the logic, and Breazy handles the batching, pooling, and rendering efficiency under the hood.

## Why use it?
- No ECS Required: Write standard Rust code. No need to learn Queries, Commands, or Bundles.
- Immediate Mode API: Draw sprites, shapes, and text with a single function call every frame (e.g., ctx.draw.sprite(...)).
- Batteries Included: Windowing, Input, and Audio are all unified into a single Context.
- Powered by Bevy: Underneath the simple API is the industrial-strength Bevy engine, giving you performance and reliability without the boilerplate.

## Current State
Breeze is in early development. The core immediate-mode drawing API is functional, but many features are still being built out.

## Example
```rust
use breeze::prelude::*;

struct MyGame {
    time: f32,
}

impl Game for MyGame {
    fn update(&mut self, ctx: &mut Context) {
        self.time = ctx.time.elapsed_secs();
    }

    fn draw(&mut self, ctx: &mut DrawContext) {

        // Draw a Circle that moves
        let x_pos = self.time.sin() * 200.0;
        ctx.circle(x_pos, 0.0, 60.0, Color::from(RED));

        // Draw a static Rectangle
        ctx.rect(0.0, -200.0, 400.0, 50.0, Color::from(BLUE_VIOLET));
        ctx.text("Hello, World!", -0.0, -200.0);

        // Draw a Ring that pulses
        let ring_radius = 100.0 + (self.time * 5.0).sin() * 25.0;
        ctx.ring(x_pos, 0.0, ring_radius, 10.0, Color::from(YELLOW));
    }
}

fn main() {
    let config = AppConfig {
        title: "Hello Window".to_string(),
        width: 1280,
        height: 720,
    };
    let my_game = MyGame { time: 0.0 };
    run(config, my_game);
}
```
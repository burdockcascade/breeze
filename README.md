[![Project Status: Alpha](https://img.shields.io/badge/Project%20Status-Alpha-yellow.svg)](https://en.wikipedia.org/wiki/Software_release_life_cycle#Alpha)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](#LICENSE)
[![crates.io](https://img.shields.io/crates/v/breeze.svg)](https://crates.io/crates/breeze)
[![tests](https://github.com/burdockcascade/breeze/actions/workflows/build-examples.yml/badge.svg?branch=master)](https://github.com/burdockcascade/bytevm/actions/workflows/build-examples.yml)
[![docs](https://docs.rs/bytevm/badge.svg)](https://docs.rs/breeze)
[![crates](https://img.shields.io/crates/d/breeze.svg)](https://crates.io/crates/breeze)

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

struct MyGame;

impl Game for MyGame {
    fn init(&mut self, _ctx: &mut Context) {
    }

    fn update(&mut self, _ctx: &mut Context) {
    }

    fn draw(&mut self, ctx: &mut DrawContext) {
        ctx.with_layer(0, |ui| {
            ui.set_camera(CameraMode::default());
            ui.text.draw("Hello, Breeze!", -100.0, 0.0);
        });
    }
}

fn main() {
    run(AppConfig::default(), MyGame);
}
```

# License
Breeze is licensed under the MIT License. See the LICENSE file for details.
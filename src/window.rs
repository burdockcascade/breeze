use bevy::prelude::*;
use bevy::window::WindowMode;

pub struct WindowContext<'a> {
    pub(crate) window: &'a mut Window,
}

impl<'a> WindowContext<'a> {
    /// Change the window title
    pub fn set_title(&mut self, title: &str) {
        self.window.title = title.to_string();
    }

    /// Resize the window
    pub fn set_size(&mut self, width: u32, height: u32) {
        self.window.resolution.set(width as f32, height as f32);
    }

    /// Get current window dimensions as a Vec2
    pub fn size(&self) -> Vec2 {
        Vec2::new(self.window.resolution.width(), self.window.resolution.height())
    }

    /// Toggle between Windowed and Borderless Fullscreen
    pub fn toggle_fullscreen(&mut self) -> bool {
        match self.window.mode {
            WindowMode::Windowed => {
                self.window.mode = WindowMode::BorderlessFullscreen(MonitorSelection::Primary);
                true
            }
            _ => {
                self.window.mode = WindowMode::Windowed;
                false
            }
        }
    }

    /// Check if the window is currently focused
    pub fn is_focused(&self) -> bool {
        self.window.focused
    }
}
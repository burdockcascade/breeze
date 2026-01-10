use bevy::prelude::*;

pub struct InputContext<'a> {
    pub(crate) keys: &'a ButtonInput<KeyCode>,
    pub(crate) mouse_buttons: &'a ButtonInput<MouseButton>,
    pub(crate) cursor_world_pos: Vec2, // We calculate this once per frame in lib.rs
}

impl<'a> InputContext<'a> {

    /// Returns true while the key is held down
    pub fn key_down(&self, key: KeyCode) -> bool {
        self.keys.pressed(key)
    }

    /// Returns true only on the frame the key was pressed
    pub fn key_pressed(&self, key: KeyCode) -> bool {
        self.keys.just_pressed(key)
    }

    /// Returns true only on the frame the key was released
    pub fn key_released(&self, key: KeyCode) -> bool {
        self.keys.just_released(key)
    }

    /// Returns the mouse position in World Space (0,0 is center of screen)
    pub fn mouse_pos(&self) -> Vec2 {
        self.cursor_world_pos
    }

    /// Returns true while the mouse button is held down
    pub fn mouse_down(&self, button: MouseButton) -> bool {
        self.mouse_buttons.pressed(button)
    }

    /// Returns true only on the frame the mouse button was pressed
    pub fn mouse_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons.just_pressed(button)
    }
}
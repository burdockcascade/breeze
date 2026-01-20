use crate::context::{Context, DrawContext};

/// 1. The States
pub enum SceneTransition {

    /// Do nothing, stay in the current scene
    None,

    /// Switch the current scene with a new one
    Switch(Box<dyn Scene>),

    /// Push a new scene onto the stack
    Push(Box<dyn Scene>),

    /// Pop the current scene off the stack
    Pop,

    /// Quit the application
    Quit,
}

/// 2. The Interface
pub trait Scene: Send + Sync {

    /// Init is called when the scene is first added to the stack
    fn init(&mut self, _ctx: &mut Context) {}

    /// Update is called only for the top-most scene in the stack
    fn update(&mut self, _ctx: &mut Context) -> SceneTransition {
        SceneTransition::None
    }

    /// Draw is called for all scenes in the stack, from bottom to top
    fn draw(&mut self, ctx: &mut DrawContext);
}

/// 3. The Logic (SceneManager)
pub struct SceneManager {
    pub(crate) stack: Vec<Box<dyn Scene>>,
    pub(crate) should_quit: bool,
}

impl SceneManager {
    pub fn new(initial_scene: impl Scene + 'static) -> Self {
        Self {
            stack: vec![Box::new(initial_scene)],
            should_quit: false,
        }
    }

    /// Helper to handle the update of the top-most scene
    pub fn update(&mut self, ctx: &mut Context) {
        // We only update the active (top) scene
        let transition = if let Some(active_scene) = self.stack.last_mut() {
            active_scene.update(ctx)
        } else {
            SceneTransition::None
        };

        // Handle the result immediately
        match transition {
            SceneTransition::None => {}
            SceneTransition::Switch(mut new_scene) => {
                new_scene.init(ctx);
                self.stack.pop();
                self.stack.push(new_scene);
            }
            SceneTransition::Push(mut new_scene) => {
                new_scene.init(ctx);
                self.stack.push(new_scene);
            }
            SceneTransition::Pop => {
                self.stack.pop();
            }
            SceneTransition::Quit => {
                self.should_quit = true;
            }
        }
    }

    /// Helper to handle drawing all scenes (Painter's Algorithm)
    pub fn draw(&mut self, ctx: &mut DrawContext) {
        for scene in self.stack.iter_mut() {
            scene.draw(ctx);
        }
    }
}
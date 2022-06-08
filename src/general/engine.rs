use std::{
    collections::{HashMap},
};
use glium::{
    glutin::{
        event_loop::{EventLoop},
    },
};
use crate::{
    general::{Context, Scene, error::GameResult},
    graphics::{Window},
};


/// The Engine is the meat and potatoes
/// of the program. The end-user should
/// only have to create this and pass in 
/// a map of scenes. Then, call the `run()`
/// function.
pub struct Engine {
    scene_map: HashMap<String, SceneManager>,
    scene_stack: Vec<String>,
    running: bool,
}

impl Engine {
    /// Create a new Engine.
    pub fn new(scene_map: HashMap<String, Scene>, starting_scene: &str) -> Engine {
        return Engine {
            scene_map,
            scene_stack: vec![starting_scene.to_string()],
            running: false,
        }
    }

    /// This will run the Engine and start the event loop on
    /// the Window.
    pub fn run(mut self) -> GameResult {
        self.running = true;
        let event_loop = EventLoop::new();
        let ctx = Context::new(&event_loop);

        Window::process(ctx, event_loop, self)?;

        return Ok(());
    }

    /// Update the current scene.
    pub fn update(&mut self, ctx: &mut Context) -> GameResult {
        let scene = self.scene();

        scene.update(ctx)?;
        scene.render(ctx)?;
        // Window::render(ctx, );

        return Ok(());
    }

    /// Get current scene.
    pub fn scene(&mut self) -> &mut Scene {
        let scene: &mut Scene = self.scene_map.get_mut(&self.scene_stack[0]).unwrap();
        return scene;
    }
}
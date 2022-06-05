use std::{
    collections::{HashMap},
};
use glium::{
    glutin::{
        event_loop::{EventLoop},
    },
};
use crate::{
    general::{Context, Scene},
    graphics::{Window},
};


/// The Engine is the meat and potatoes
/// of the program. The end-user should
/// only have to create this and pass in 
/// a map of scenes. Then, call the `run()`
/// function.
pub struct Engine {
    scene_map: HashMap<String, Box<dyn Scene>>,
    scene_stack: Vec<String>,
    running: bool,
}

impl Engine {
    /// Create the Engine.
    pub fn new(scene_map: HashMap<String, Box<dyn Scene>>, starting_scene: &str) -> Engine {
        return Engine {
            scene_map,
            scene_stack: vec![starting_scene.to_string()],
            running: false,
        }
    }

    /// Run the Engine. This houses the 
    /// updating and rendering of the game.
    pub fn run(&mut self) {
        self.running = true;
        let event_loop = EventLoop::new();
        let mut ctx = Context::new(&event_loop);

        Window::update(&mut ctx, event_loop);
    }

    pub fn scene(&mut self) -> &Box<dyn Scene> {
        let scene: &Box<dyn Scene> = self.scene_map.get(&self.scene_stack[0]).unwrap();
        return scene;
    }
}
use glium::{
    glutin::{
        event_loop::{EventLoop},
    },
};
use crate::{
    general::{Context},
    graphics::{Window},
};


/// The Engine is the meat and potatoes
/// of the program. The end-user should
/// only have to create this and pass in 
/// a map of scenes. Then, call the `run()`
/// function.
pub struct Engine {

}

impl Engine {
    /// Create the Engine.
    pub fn new() -> Engine {
        return Engine {

        }
    }

    /// Run the Engine. This houses the 
    /// updating and rendering of the game.
    pub fn run(&mut self) {
        let event_loop = EventLoop::new();
        let mut ctx = Context::new(&event_loop);

        Window::update(&mut ctx, event_loop);
    }
}
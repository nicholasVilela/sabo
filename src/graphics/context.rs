use glium::{
    glutin::{
        event_loop::{EventLoop},
    },
};
use crate::{
    graphics::{Window},
};


/// Graphics Context controls the Window. Will be used to
/// draw to the screen.
pub struct GraphicsContext {
    pub window: Window,
}

impl GraphicsContext {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let window = Window::new(&event_loop);

        return Self {
            window,
        }
    }
}
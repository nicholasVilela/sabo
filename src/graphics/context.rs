use glium::{
    glutin::{
        event_loop::{EventLoop},
    },
};
use crate::{
    graphics::{Window},
};


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
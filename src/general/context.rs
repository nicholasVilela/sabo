use glium::{
    glutin::{
        event_loop::{EventLoop},
    },
};
use crate::{
    graphics::{GraphicsContext},
};


pub struct Context {
    pub graphics_context: GraphicsContext,
    pub running: bool,
}
impl Context {
    pub fn new(event_loop: &EventLoop<()>) -> Self {
        let graphics_context = GraphicsContext::new(event_loop);

        return Self {
            graphics_context,
            running: true,
        }
    }
}
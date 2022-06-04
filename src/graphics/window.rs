use glium::{
    glutin::{
        event_loop::{EventLoop},
        window::{WindowBuilder},
        ContextBuilder,
    },
    Display,
};
use crate::{
    general::{Context},
};


pub struct Window {
    display: Display,
}

impl Window {
    pub fn new(event_loop: &EventLoop<()>) -> Window {
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, event_loop).unwrap();

        return Window {
            display,
        }
    }

    pub fn update(ctx: &mut Context, event_loop: EventLoop<()>) {
        event_loop.run(move |mut event, _, control_flow| {
            println!("ahh");
        });
    }

    pub fn render(ctx: &mut Context) {

    }
}
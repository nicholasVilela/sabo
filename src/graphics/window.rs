use glium::{
    glutin::{
        event_loop::{EventLoop, ControlFlow},
        event::{Event, WindowEvent},
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
        event_loop.run(move |mut ev, _, control_flow| {
            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    },
                    _ => return,
                },
                _ => (),
            }
        });
    }

    pub fn display(&mut self) -> &mut Display {
        return &mut self.display;
    }
}
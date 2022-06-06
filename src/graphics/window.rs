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
    general::{Context, Scene, Engine},
};


/// Window keeps track of the screen and updates the event loop.
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

    pub fn update(mut ctx: Context, event_loop: EventLoop<()>, mut engine: Engine) {
        event_loop.run(move | ev, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match ev {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    },
                    _ => return,
                },
                Event::MainEventsCleared => {
                    engine.scene().update(&mut ctx);
                    engine.scene().render(&mut ctx);
                },
                _ => (),
            }
        });
    }

    pub fn display(&mut self) -> &mut Display {
        return &mut self.display;
    }
}
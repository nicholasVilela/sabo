use glium::{
    glutin::{
        event_loop::{EventLoop, ControlFlow},
        event::{Event, WindowEvent},
        window::{WindowBuilder},
        ContextBuilder,
    },
    Display,
    Frame,
};
use crate::{
    general::{Context, Scene, Engine, error::GameResult},
    graphics::{Canvas},
};


/// Window keeps track of the screen and updates the event loop.
pub struct Window {
    display: Display,
}

impl Window {
    /// Creates a new Window.
    pub fn new(event_loop: &EventLoop<()>) -> Window {
        let wb = WindowBuilder::new();
        let cb = ContextBuilder::new();
        let display = Display::new(wb, cb, event_loop).unwrap();

        return Window {
            display,
        }
    }

    /// Starts the Event Loop and updates the Engine.
    pub fn process(mut ctx: Context, event_loop: EventLoop<()>, mut engine: Engine) -> GameResult {
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
                    engine.update(&mut ctx);
                },
                _ => (),
            }
        });
    }

    pub fn render(ctx: &mut Context, canvas: &Canvas) -> GameResult {
        let window = ctx.graphics_context.window;
        let mut target = window.display.draw();
        // target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.set_finish().unwrap();

        return Ok(());
    }

    /// Get a mutable reference to the Window's Display.
    pub fn display(&mut self) -> &mut Display {
        return &mut self.display;
    }
}
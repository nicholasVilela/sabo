use glium::{
    Surface,
    Frame,
};
use crate::{
    general::{error::GameResult, Context},
    graphics::{Texture},
};


/// Something you can render to.
pub struct Canvas {
    // texture: Texture,
}

impl Canvas {
    /// Create a new Canvas.
    pub fn new(ctx: &mut Context) -> GameResult<Canvas> {
        // let texture = Texture::new();
        let canvas = Canvas {
            // texture,
        };

        return Ok(canvas);
    }

    pub fn render(&mut self) -> GameResult {
        
        
        return Ok(());
    }
}
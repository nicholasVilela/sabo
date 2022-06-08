use crate::{
    general::{Context,   error::GameResult},
};

mod context;
pub use context::*;

mod canvas;
pub use canvas::*;

mod texture;
pub use texture::*;

mod vertex;
pub use vertex::*;

mod window;
pub use window::*;


pub fn draw(ctx: &mut Context, target: &Canvas) -> GameResult {
    ctx.graphics_context.window.render(target);

    return Ok(());
}
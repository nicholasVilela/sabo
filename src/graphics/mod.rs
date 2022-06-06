use crate::{
    general::{error::GameResult},
};

mod context;
pub use context::*;

mod surface;
pub use surface::*;

mod window;
pub use window::*;



pub fn draw() -> GameResult {
    return Ok(());
}
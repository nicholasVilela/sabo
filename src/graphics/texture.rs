use glium::{
    Surface,
};
use crate::{
    general::{error::GameResult},
};


pub struct Texture {
    dimensions: (u32, u32),
}

impl Texture {
    pub fn new(dimensions: (u32, u32)) -> GameResult<Texture> {
        let texture = Texture {
            dimensions,
        };

        return Ok(texture);
    }
}

// impl Surface for Texture {
//     fn clear(&mut self, Rect: Option<&Rect>, color: Option<(f32, f32, f32, f32)>, color_srgb: bool, depth: Option<f32>, stencil: Option<i32>){
//         ops::clear(&self.)
//     }
// }
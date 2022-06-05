use crate::{
    general::{Context},
};


/// A Scene is a separated piece of logic to be run
/// in the Engine. There can only be one Scene running
/// at a time.
pub trait SceneTrait {
    fn update(&mut self, ctx: &mut Context) {}
    fn fixed_update(&mut self, ctx: &mut Context) {}
    fn render(&mut self, ctx: &mut Context) {}
}

pub type Scene = Box<dyn SceneTrait>;
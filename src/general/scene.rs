use crate::{
    general::{Context},
};


/// Wrapper type for `Box<dyn SceneTrait>`.
pub type Scene = Box<dyn SceneTrait>;

/// Trait to be implemented for custom scenes.
pub trait SceneTrait {
    fn update(&mut self, ctx: &mut Context);
    fn fixed_update(&mut self, ctx: &mut Context);
    fn render(&mut self, ctx: &mut Context);
}

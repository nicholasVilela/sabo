use std::{
    collections::{HashMap},
};
use crate::{
    general::{Context, error::GameResult},
    graphics::{Canvas},
};


/// Wrapper type for `Box<dyn SceneTrait>`.
pub type Scene = Box<dyn SceneTrait>;

/// Trait to be implemented for custom scenes.
pub trait SceneTrait {
    fn update(&mut self, ctx: &mut Context) -> GameResult;
    fn fixed_update(&mut self, ctx: &mut Context) -> GameResult;
    fn render(&mut self, ctx: &mut Context) -> GameResult;
}

pub struct SceneManager<A: SceneTrait> {
    scene: A,
    layers: HashMap<String, Canvas>,
}

impl<A: SceneTrait> SceneManager<A> {
    
}
use std::{
    collections::{HashMap},
};
use sabo::{
    general::{Scene, SceneTrait, Context, error::GameResult},
    graphics::{Canvas},
};


pub struct Game {
    pub layer_map: HashMap<String, Canvas>
}

impl Game {
    pub fn new() -> GameResult<Self> {
        let layer_map = Game::construct_layer_map()?;

        let game = Self {
            layer_map,
        };

        return Ok(game);
    }

    pub fn construct_layer_map() -> GameResult<HashMap<String, Canvas>> {
        let layer_map = HashMap::new();

        return Ok(layer_map);
    }
}

impl SceneTrait for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        println!("Updating Game Scene");

        return Ok(());
    }

    fn fixed_update(&mut self, ctx: &mut Context) -> GameResult {
        return Ok(());
    }

    fn render(&mut self, ctx: &mut Context) -> GameResult {
        for (_key, layer) in &mut self.layer_map {
            
            layer.render()?;
        }

        return Ok(());
    }
}
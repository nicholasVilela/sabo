use std::{
    collections::{HashMap},
};
use sabo::{
    general::{Engine, Scene},
};

mod scenes;
pub use scenes::*;


fn main() {
    let game: Box<dyn Scene> = Box::new(Game::new());

    let mut scene_map = HashMap::new();
    scene_map.insert("game".to_string(), game);

    let mut engine = Engine::new(scene_map, "game");
    engine.run();
}

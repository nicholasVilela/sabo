use crate::{
    general::{error::{GameResult}},
};


pub struct EngineConfig {

}

impl EngineConfig {
    pub fn new() -> GameResult<EngineConfig> {
        let engine_config = EngineConfig {};

        return Ok(engine_config);
    }
}
use crate::{
    general::{Engine},
};

#[test]
fn create_engine() {
    let mut engine = Engine::new();
    engine.run();
}
use crate::{
    general::{Engine},
};

#[test]
fn create_engine() {
    let mut engine = Engine::new();
    engine.run();

    assert_eq!(2, 2);
}
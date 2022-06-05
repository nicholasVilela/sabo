use sabo::{
    general::{Scene, SceneTrait, Context},
};


pub struct Game {

}

impl Game {
    pub fn new() -> Self {
        return Self {

        }
    }
}

impl SceneTrait for Game {
    fn update(&mut self, ctx: &mut Context) {
        println!("Updating Game Scene");
    }

    fn fixed_update(&mut self, ctx: &mut Context) {

    }

    fn render(&mut self, ctx: &mut Context) {
        println!("Rendering Game Scene");
    }
}
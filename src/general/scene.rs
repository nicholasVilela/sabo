/// A Scene is a separated piece of logic to be run
/// in the Engine. There can only be one Scene running
/// at a time.
pub trait Scene {
    fn update(&mut self) {}
    fn fixed_update(&mut self) {}
    fn render(&mut self) {}
}
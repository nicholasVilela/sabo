use crate::{
    general::{error::GameResult},
};


/// Something that can be drawn on. This should
/// eventually be drawn onto the Window display.
pub trait Surface {
    fn render() -> GameResult {
        return Ok(());
    }
}
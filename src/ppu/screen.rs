use super::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::NES;
pub trait Screen {
    fn render_pixel(&mut self, x: u16, y: u16, pixel: (u8, u8, u8));
}

impl<'a, S: Screen> NES<'a, S> {
    fn render_bg(&mut self) {
        for x in 0..SCREEN_WIDTH {
            for y in 0..SCREEN_HEIGHT {
                //TODO
            }
        }
    }
}

use super::{PPU, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::NES;
pub trait Screen {
    fn render_pixel(&mut self, x: u16, y: u16, pixel: (u8, u8, u8));
}
impl<'a> PPU<'a> {
    pub fn render_pixel(&mut self, x: u16, y: u16) -> Option<(u8, u8, u8)> {
        let nametables_base = self.regs.nametable_addr();
        let chr_base = self.regs.bg_chr_addr();
        let nametable = self.loadb(nametables_base + ((x >> 3) + (y >> 3) * 0x20) as u16);
        let chr_addr = chr_base + ((nametable as u16) << 4) + (y & 0x7);
        let x_offset = (!x) & 0x7;
        let low = ((self.loadb(chr_addr) >> x_offset) & 0x1)
            | (((self.loadb(chr_addr + 8) >> x_offset) & 0x1) << 1);
        let attr = self.loadb(nametables_base + 32 * 30 + ((x >> 5) + (y >> 5) * 8) as u16);
        let attr_offset = ((x & 0x10) >> 3) | ((y & 0x10) >> 2);
        let high = (attr >> attr_offset) & 0x3;
        let index = low | (high << 2);
        self.palette.get_color_by_index(index)
    }
}

impl<'a, S: Screen> NES<'a, S> {
    fn render_bg(&mut self) {
        for x in 0..SCREEN_WIDTH as u16 {
            for y in 0..SCREEN_HEIGHT as u16 {
                if let Some(pixel) = self.cpu.mem.ppu.render_pixel(x, y) {
                    self.screen.render_pixel(x, y, pixel)
                }
            }
        }
    }
}

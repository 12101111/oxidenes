mod nametable;
mod palette;
mod reg;
mod screen;
use self::{nametable::*, palette::Palette, reg::Regs};
use crate::mapper::CHR;
pub use screen::Screen;

pub const SCREEN_WIDTH: usize = 256;
pub const SCREEN_HEIGHT: usize = 240;

pub const CYCLES_PER_SCANLINE: usize = 114; // 29781 cycles per frame / 261 scanlines = 113.667
pub const VBLANK_SCANLINE: u16 = 240;
pub const LAST_SCANLINE: u16 = 261;

pub struct PPU<'a> {
    /// CPU:0x2000-0x2007
    pub regs: Regs,
    /// PPU
    /// 0x1000 pattern table 图样表CHR-ROM
    /// 0x0000-0x1000 pattern table 0  
    /// 0x1000-0x2000 pattern table 1
    chr: CHR<'a>,
    /// 0x3C0 name table名称表 32*30 8*8 => 256*240
    /// 0x40 attribute table属性表
    pub nametables: NameTable,
    /// 0x3F00-0x3F10 image palette index 调色板索引
    /// 0x3F10-0x3F20 sprite palette index
    palette: Palette,
    /// Object Attribute Memory is a internal memory inside PPU
    /// not in PPU address space
    pub oam: [u8; 0x100],
    /// Pre-render scanline (-1, 261) 2行 Vblank结束
    /// Visible scanlines (0-239) 240行可见扫描线
    /// Post-render scanline (240) 1行VBlank开始
    /// Vertical blanking lines (241-260) 20行的VBlank线.
    pub scanline: u16,
    pub cycle: usize,
}

impl<'a> PPU<'a> {
    pub fn loadb(&self, addr: u16) -> u8 {
        // 0x4000-0xC000 is mirror of 0x0000-0x4000
        let addr = addr & 0x3FFF;
        match addr {
            0x0000...0x1FFF => self.chr[addr],
            0x2000...0x2FFF => self.nametables[addr],
            // 0x3000-0x3F00 is mirror of 0x2000-0x2F00
            0x3000...0x3EFF => self.loadb(addr - 0x1000),
            // 0x3F20-0x4000 is mirror of 0x3F00-0x3F20
            0x3F00...0x3FFF => self.palette[addr],
            _ => unreachable!(),
        }
    }
    pub fn storeb(&mut self, addr: u16, val: u8) {
        let addr = addr & 0x3FFF;
        match addr {
            0x0000...0x1FFF => self.chr.storeb(addr, val),
            0x2000...0x2FFF => self.nametables[addr] = val,
            0x3000...0x3EFF => self.storeb(addr - 0x1000, val),
            0x3F00...0x3FFF => self.palette[addr] = val,
            _ => unreachable!(),
        }
    }
    pub fn new(chr: CHR<'a>) -> PPU {
        PPU {
            chr,
            regs: Regs::new(),
            nametables: NameTable::new(),
            palette: Palette::new(),
            oam: [0u8; 0x100],
            scanline: 0,
            cycle: 0,
        }
    }
}

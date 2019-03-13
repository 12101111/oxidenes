use crate::mapper::CHR;
mod reg;

use self::{reg::Regs};

pub struct PPU<'a> {
    pub regs: Regs,
    chr: CHR<'a>,
    pub oam: [u8;0x100],
}

impl<'a> PPU<'a>{
    pub fn new(chr:CHR<'a>)->PPU<'a>{
        PPU{
            regs:Regs::new(),
            chr,
            oam:[0u8;0x100],
        }
    }
    pub fn reset(&mut self){
        self.regs.reset();
        self.oam.iter_mut().for_each(|x| *x=0);
    }
}
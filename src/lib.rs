#![no_std]
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;
use mos6502::CPU;

mod apu;
mod input;
mod mapper;
mod mem;
mod ppu;
mod rom;

use mem::NESMemory;
pub use ppu::{Screen, SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct NES<'a, S: Screen> {
    cpu: CPU<NESMemory<'a>>,
    screen: S,
}

impl<'a, S: Screen> NES<'a, S> {
    pub fn new(buffer: &'a [u8], screen: S) -> NES<'a, S> {
        let mem = NESMemory::new(buffer);
        NES {
            cpu: CPU::new(mem),
            screen,
        }
    }
    pub fn frame(&mut self) {
        assert_eq!(self.cpu.mem.ppu.scanline, 0);
        assert_eq!(self.cpu.mem.ppu.cycles, 0);
        loop {}
    }
    #[cfg(feature = "disasm")]
    pub fn step(&mut self) {
        self.cpu.execute();
    }
    #[cfg(feature = "disasm")]
    pub fn set_pc(&mut self, pc: u16) {
        self.cpu.regs.pc = pc;
    }
    #[cfg(feature = "disasm")]
    pub fn get_cycles(&self) -> usize {
        self.cpu.mem.get_cycles()
    }
}

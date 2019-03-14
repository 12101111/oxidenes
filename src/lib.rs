#![no_std]
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate log;
use mos6502::{Memory, CPU};

mod apu;
mod input;
mod mapper;
mod ppu;
mod rom;

use apu::APU;
use input::Input;
use mapper::PRG;
use ppu::PPU;
pub use ppu::{Screen, SCREEN_HEIGHT, SCREEN_WIDTH};
use rom::Rom;

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

struct NESMemory<'a> {
    /// $0000 	$0800 		RAM
    /// $0800 	$1000 	M 	RAM
    /// $1000 	$1800 	M 	RAM
    /// $1800 	$2000 	M 	RAM
    ram: [u8; 0x800],
    /// $2000 	$2008 		Registers   PPU
    /// $2008 	$4000 	R 	Registers   8Bit Mirror of 2000-2008
    ppu: PPU<'a>,
    /// $4000 	$4020 		Registers   APU
    apu: APU,
    input: Input,
    /// $4020 	$6000		Expansion ROM
    /// $6000 	$8000 		SAVERAM
    sram: [u8; 0x2000],
    /// $8000 	$C000 		PRG-ROM
    /// $C000 	$10000 		PRG-ROM
    prg: PRG<'a>,
    cycles: usize,
}

impl<'a> Memory for NESMemory<'a> {
    fn reset(&mut self) {
        self.ram = [0; 0x800];
        // NES don't reset PPU when reset
        //self.ppu.reset();
        self.apu.reset();
        self.input.reset();
        self.cycles = 7;
    }
    fn loadb(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000...0x1FFF => self.ram[addr as usize & 0x7ff],
            0x2000...0x3FFF => self.ppu.reg_loadb(addr),
            0x4000...0x4013 => self.apu.loadb(addr),
            0x4014 => unimplemented!("Read DMA"),
            0x4015 => self.apu.get_channel(),
            0x4016 => self.input.load1(),
            0x4017 => self.input.load2(),
            0x4018...0x401F => unimplemented!("TEST MODE"),
            0x4020...0x5FFF => unimplemented!("Read Expansion ROM"),
            0x6000...0x7FFF => self.sram[addr as usize & 0x1FFF],
            0x8000...0xFFFF => self.prg[addr],
        }
    }
    fn try_loadb(&self, addr: u16) -> Option<u8> {
        match addr {
            0x0000...0x1FFF => Some(self.ram[addr as usize & 0x7ff]),
            0x2000...0x5FFF => None,
            0x6000...0x7FFF => Some(self.sram[addr as usize & 0x1FFF]),
            0x8000...0xFFFF => Some(self.prg[addr]),
        }
    }
    fn storeb(&mut self, addr: u16, val: u8) {
        match addr {
            0x0000...0x1FFF => self.ram[addr as usize & 0x7ff] = val,
            0x2000...0x3FFF => self.ppu.reg_storeb(addr, val),
            0x4000...0x4013 => self.apu.storeb(addr, val),
            0x4014 => self.dma(val),
            0x4015 => self.apu.set_channel(val),
            0x4016 => self.input.set(val),
            0x4017 => self.apu.set_mode(val),
            0x4018...0x401F => unimplemented!("TEST MODE"),
            0x4020...0x5FFF => unimplemented!("Write Expansion ROM"),
            0x6000...0x7FFF => self.sram[addr as usize & 0x1FFF] = val,
            0x8000...0xFFFF => unimplemented!("Write to PGR"),
        }
    }
    fn add_cycles(&mut self, val: usize) {
        self.cycles += val
    }
    fn get_cycles(&self) -> usize {
        self.cycles
    }
}

impl<'a> NESMemory<'a> {
    fn new(buffer: &'a [u8]) -> NESMemory<'a> {
        let rom = Rom::load(&buffer);
        let (prg, chr, header) = rom.split();
        info!("Load Rom:{}", header);
        NESMemory {
            ram: [0; 0x800],
            ppu: PPU::new(chr),
            apu: APU::new(),
            input: Input::new(),
            sram: [0; 0x2000],
            prg,
            cycles: 7,
        }
    }

    fn dma(&mut self, addr_high: u8) {
        let start = (addr_high as usize) << 8;
        let dst = match addr_high {
            0x00...0x1F => &self.ram[start..(start + 0x100)],
            0x60...0x7F => &self.sram[start..(start + 0x100)],
            0x80...0xFF => &self.prg[start as u16..(start as u16 + 0x100)],
            _ => unimplemented!(),
        };
        self.ppu.oam.copy_from_slice(dst);
        self.add_cycles(if self.cycles % 2 == 0 { 513 } else { 514 });
    }
}

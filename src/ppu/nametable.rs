use core::ops::{Index, IndexMut};
pub struct NameTable {
    inner: [u8; 0x800],
}
impl NameTable {
    pub fn new() -> NameTable {
        NameTable {
            inner: [0u8; 0x800],
        }
    }
    pub fn addr(&self, addr: u16) -> usize {
        let addr = addr as usize;
        addr & 0x3FF + if addr < 0x2800 { 0 } else { 0x400 }
    }
}

impl Index<u16> for NameTable {
    type Output = u8;
    fn index(&self, index: u16) -> &u8 {
        &self.inner[self.addr(index)]
    }
}

impl IndexMut<u16> for NameTable {
    fn index_mut(&mut self, index: u16) -> &mut u8 {
        let addr = self.addr(index);
        &mut self.inner[addr]
    }
}

use core::ops::{Index, Range};
pub struct PRG<'a> {
    inner: &'a [u8],
    mapper: u16,
}
impl<'a> Index<u16> for PRG<'a> {
    type Output = u8;
    fn index(&self, index: u16) -> &u8 {
        assert!(index >= 0x8000);
        match self.mapper {
            _ => {
                if self.inner.len() > 0x4000 {
                    //32KB
                    //$8000-$FFFF
                    &self.inner[index as usize & 0x7fff]
                } else {
                    //16KB
                    //$8000-$BFFF
                    &self.inner[index as usize & 0x3fff]
                }
            }
        }
    }
}
impl<'a> Index<Range<u16>> for PRG<'a> {
    type Output = [u8];
    fn index(&self, index: Range<u16>) -> &[u8] {
        assert!(index.start >= 0x8000);
        match self.mapper {
            _ => {
                if self.inner.len() > 0x4000 {
                    //32KB
                    //$8000-$FFFF
                    &self.inner[index.start as usize & 0x7FFF..index.end as usize & 0x7FFF]
                } else {
                    //16KB
                    //$8000-$BFFF
                    &self.inner[index.start as usize & 0x3FFF..index.end as usize & 0x3FFF]
                }
            }
        }
    }
}
impl<'a> PRG<'a> {
    pub fn new(inner: &'a [u8], mapper: u16) -> PRG<'a> {
        PRG { inner, mapper }
    }
    pub fn storeb(&mut self, addr: u16, val: u8) {
        match self.mapper {
            _ => unimplemented!("Read Only {:04X}={:02X}", addr, val),
        }
    }
}
pub struct CHR<'a> {
    inner: &'a [u8],
    mapper: u16,
}
impl<'a> Index<u16> for CHR<'a> {
    type Output = u8;
    fn index(&self, index: u16) -> &u8 {
        match self.mapper {
            _ => &self.inner[index as usize],
        }
    }
}
impl<'a> CHR<'a> {
    pub fn new(inner: &'a [u8], mapper: u16) -> CHR<'a> {
        CHR { inner, mapper }
    }
    pub fn storeb(&mut self, addr: u16, val: u8) {
        match self.mapper {
            _ => unimplemented!("Read Only ${:04X} = {:02X} ", addr, val),
        }
    }
}
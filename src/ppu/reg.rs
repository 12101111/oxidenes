use super::PPU;

pub struct Regs{}

impl Regs{
    pub fn new()->Regs{
        warn!("TODO");
        Regs{}
    }
    pub fn reset(&mut self){
        unimplemented!()
    }
}

impl<'a> PPU<'a>{
    pub fn reg_loadb(&mut self,addr:u16)->u8{
        warn!("TODO");
        0
    }
    pub fn reg_storeb(&mut self,addr:u16,val:u8){
        warn!("TODO")
    }
}
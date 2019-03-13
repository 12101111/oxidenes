pub struct APU{}
impl APU{
    pub fn new()->APU{
        APU{}
    }
    pub fn reset(&mut self){
        warn!("TODO");
    }
    pub fn loadb(&mut self,addr:u16)->u8{
        warn!("TODO");
        0
    }
    pub fn storeb(&mut self,addr:u16,val:u8){
        warn!("TODO");
    }
    pub fn get_channel(&mut self)->u8{
        warn!("TODO");
        0
    }
    pub fn set_channel(&mut self,val:u8){
        warn!("TODO");
    }
    pub fn set_mode(&mut self,val:u8){
        warn!("TODO");
    }
}
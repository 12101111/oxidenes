use super::PPU;
bitflags! {
    /// PPUCTRL $2000  VPHB SINN Write Only
    struct PPUCTRL:u8{
        /// 7 NMI enable 在VBlank时触发NMI
        const V = 1 << 7;
        /// 6 PPU master/slave Not Used
        const P = 1 << 6;
        /// 5 sprite height,精灵尺寸(高度) 0(8x8) 1(8x16)
        const H = 1 << 5;
        /// 4 Background Pattern Table Address 0($0000) 1($1000)
        const B = 1 << 4;
        /// 3 Sprite Pattern Table Address 0($0000) 1($1000) (ignore when H=1)
        const S = 1 << 3;
        /// 2 increment mode  PPU读写显存地址增量 0(+1 列模式) 1(+32 行模式)
        const I = 1 << 2;
        /// 0,1 Name Table Address 00($2000) 01($2400) 10($2800) 11($2C00)
        const NH = 1 << 1;
        const NL = 1 << 0;
    }
}
impl Regs {
    #[inline]
    pub fn nmi_enable(&self) -> bool {
        self.ctrl.contains(PPUCTRL::V)
    }
    #[inline]
    pub fn sprite_height(&self) -> u8 {
        if self.ctrl.contains(PPUCTRL::H) {
            16
        } else {
            8
        }
    }
    #[inline]
    pub fn sp_chr_addr(&self) -> u16 {
        if self.ctrl.contains(PPUCTRL::S) {
            0x1000
        } else {
            0x0
        }
    }
    #[inline]
    pub fn bg_chr_addr(&self) -> u16 {
        if self.ctrl.contains(PPUCTRL::B) {
            0x1000
        } else {
            0x0
        }
    }
    #[inline]
    pub fn nametable_addr(&self) -> u16 {
        0x2000 + ((self.ctrl.bits() as u16 & 0x3) << 10)
    }
}
bitflags! {
    /// PPUMASK $2001  BGRs bMmG Write Only
    struct PPUMASK:u8{
        /// 567 颜色强调使能标志位(当GRAY=1时显示为1的颜色,只能设置一种)
        /// NTSC B   PAL B
        const B =1 << 7;
        /// NTSC G  PAl R
        const G = 1 << 6;
        /// NTSC R  PAL G
        const R = 1 << 5;
        /// 4 sprite enable (s)精灵显示使能标志位 	1(显示精灵)
        const SE = 1 << 4;
        /// 3 background enable (b) 	背景显示使能标志位 	1(显示背景)
        const BE = 1 << 3;
        /// 2 sprite left column enable (M)精灵掩码 0(不显示最左边那列, 8像素)的精灵
        const SL = 1 << 2;
        /// 1 background left column enable (m), 背景掩码 0(不显示最左边那列, 8像素)的背景
        const BL = 1 << 1;
        /// 0 greyscale 显示模式 0(彩色) 1(灰阶)
        const GREY = 1 << 0;
    }
}

bitflags! {
    /// PPUSTATUS $2002 VSO- ---- Read Only
    struct PPUSTATUS:u8{
        /// 7 blank (V) VBlank开始时置1, 结束或者读取该字节($2002)后置0
        const V = 1 << 7;
        /// 6 sprite 0 hit (S)  	1(#0精灵命中) VBlank之后置0
        const S = 1 << 6;
        /// 5 sprite overflow (O)  0(当前扫描线精灵个数小于8) 1 精灵溢出
        const O = 1 << 5;
    }
}

impl Regs {
    #[inline]
    pub fn vblank_start(&mut self) {
        self.status.insert(PPUSTATUS::V)
    }
    #[inline]
    pub fn vblank_end(&mut self) {
        self.status.remove(PPUSTATUS::V)
    }
    #[inline]
    pub fn vblank(&self) -> bool {
        self.status.contains(PPUSTATUS::V)
    }
}

pub struct Regs {
    /// $2000 W
    ctrl: PPUCTRL,
    /// $2001 W
    mask: PPUMASK,
    /// $2002 R
    status: PPUSTATUS,
    /// $2003 W 设置OAM地址
    oam_addr: u8,
    /// $2004 R/W 读写OAM数据,访问后OAMADDR+1
    //oam_data
    /// $2005 WW 屏幕滚动偏移 第一个值: 垂直滚动 第二个值: 水平滚动
    /// $2006 WW 第一个写指针的高6位 第二个写低8位
    /// see http://wiki.nesdev.com/w/index.php/PPU_scrolling
    /// yyy NN YYYYY XXXXX
    /// ||| || ||||| +++++-- coarse X scroll
    /// ||| || +++++-------- coarse Y scroll
    /// ||| ++-------------- nametable select
    /// +++----------------- fine Y scroll
    /// Current VRAM address (15 bits)
    v: u16,
    /// Temporary VRAM address (15 bits);
    /// can also be thought of as the address of the top left onscreen tile.
    t: u16,
    /// x Fine X scroll (3 bits)
    x: u8,
    /// $2005 和 $2006 共享一个写入控制,
    /// false:要写入第一个,true:要写入第二个
    w: bool,
    /// $2007 R/W 访问显存数据 PPUADDR会在读写后+1或者+32
    ppudata_buffer: u8,
}

impl Regs {
    pub fn new() -> Regs {
        Regs {
            ctrl: PPUCTRL::empty(),
            mask: PPUMASK::empty(),
            status: PPUSTATUS::empty(),
            oam_addr: 0,
            v: 0,
            t: 0,
            x: 0,
            w: false,
            ppudata_buffer: 0,
        }
    }
}

impl<'a> PPU<'a> {
    pub fn reg_loadb(&mut self, addr: u16) -> u8 {
        assert!(addr >= 0x2000);
        assert!(addr < 0x4000);
        match addr & 0x7 {
            0 => unimplemented!("Write Only"),
            1 => unimplemented!("Write Only"),
            2 => {
                let data = self.regs.status.bits();
                self.regs.vblank_end();
                self.regs.w = false;
                data
            }
            3 => unimplemented!("Write Only"),
            4 => self.oam[self.regs.oam_addr as usize],
            5 => unimplemented!("Write Only"),
            6 => unimplemented!("Write Only"),
            7 => {
                let addr = self.regs.v;
                self.regs.v = self
                    .regs
                    .v
                    .wrapping_add(if self.regs.ctrl.contains(PPUCTRL::I) {
                        32
                    } else {
                        1
                    });
                if addr < 0x3F00 {
                    let data = self.regs.ppudata_buffer;
                    self.regs.ppudata_buffer = self.loadb(addr);
                    data
                } else {
                    self.regs.ppudata_buffer = self.loadb(addr - 0x1000);
                    self.loadb(addr)
                }
            }
            _ => unreachable!(),
        }
    }
    pub fn reg_storeb(&mut self, addr: u16, val: u8) {
        assert!(addr >= 0x2000);
        assert!(addr < 0x4000);
        match addr & 0x7 {
            0 => {
                self.regs.ctrl = PPUCTRL::from_bits_truncate(val);
                self.regs.t = (self.regs.t & (!(0x3 << 10))) | ((val as u16 & 0x3) << 10);
            }
            1 => self.regs.mask = PPUMASK::from_bits_truncate(val),
            2 => unimplemented!("READ ONLY"),
            3 => self.regs.oam_addr = val,
            4 => {
                self.oam[self.regs.oam_addr as usize] = val;
                self.regs.oam_addr = self.regs.oam_addr.wrapping_add(1);
            }
            5 => {
                if !self.regs.w {
                    self.regs.t = (self.regs.t & (!0x11)) | (val as u16 >> 3);
                    self.regs.x = val & 0x7;
                } else {
                    self.regs.t = (self.regs.t & 0xC1F)
                        | ((val as u16 & 0xF8) << 2)
                        | ((val as u16 & 0x7) << 12);
                }
                self.regs.w = !self.regs.w;
            }
            6 => {
                if !self.regs.w {
                    self.regs.t = (self.regs.t & 0xFF) | ((val as u16 & 0x3F) << 8);
                } else {
                    self.regs.t = (self.regs.t & 0xFF00) | (val as u16);
                    self.regs.v = self.regs.t;
                }
                self.regs.w = !self.regs.w;
            }
            7 => {
                let addr = self.regs.v;
                self.storeb(addr, val);
                self.regs.v = self
                    .regs
                    .v
                    .wrapping_add(if self.regs.ctrl.contains(PPUCTRL::I) {
                        32
                    } else {
                        1
                    });
            }
            _ => unreachable!(),
        }
    }
}

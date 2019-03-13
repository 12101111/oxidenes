use core::fmt;
use crate::mapper::{PRG,CHR};

pub struct Rom<'a> {
    /// 16 bytes
    pub header: NesHeader,
    //Trainer is used for cheat (0 or 512 bytes)
    // pub trainer: Option<[u8; 512]>,
    /// PRG ROM data (16384 * x bytes)
    pub prg: &'a [u8],
    /// CHR ROM data, if present (8192 * y bytes)
    pub chr: &'a [u8],
    // Some ROM-Images additionally contain a 128-byte
    // (or sometimes 127-byte) title at the end of the file.
}

impl<'a> Rom<'a> {
    pub fn split(self) -> (PRG<'a>, CHR<'a>, NesHeader) {
        let mapper = self.header.mapper();
        (
            PRG::new(self.prg, mapper),
            CHR::new(self.chr, mapper),
            self.header,
        )
    }
    pub fn load(reader: &'a [u8]) -> Rom<'a> {
        let mut bytes = 0;
        let header = &reader[0..16];
        assert_eq!(header[8], 0);
        assert_eq!(header[9], 0);
        assert_eq!(header[10], 0);
        let header = NesHeader {
            magic: [header[0], header[1], header[2], header[3]],
            prg_rom_size: header[4],
            chr_rom_size: header[5],
            flags_6: header[6],
            flags_7: header[7],
            flags_8: header[8],
            zero: [0; 7],
        };
        bytes += 16;
        if header.magic != *b"NES\x1a" {
            panic!("Can't open ROM, invalid NES Format Header");
        } else {
            if header.trainer() {
                warn!("unsupport trainer");
                bytes += 512;
            }
            let prg_bytes = header.prg_rom_size as usize * 16384;
            let (prg, chr) = (&reader[bytes..]).split_at(prg_bytes);
            assert!(chr.len() == header.chr_rom_size as usize * 8192);
            Rom {
                header,
                // trainer,
                prg,
                chr,
            }
        }
    }
}

pub struct NesHeader {
    /// "NES^Z"
    /// 'N' 'E' 'S' '\x1a'(EOF)
    pub magic: [u8; 4],
    /// PRG-ROM size in 16 KiB units
    pub prg_rom_size: u8,
    /// $5
    /// number of 8K units of CHR-ROM
    /// 以 8192(0x2000)字节作为单位的CHR-ROM大小数量
    pub chr_rom_size: u8,
    ///   D~7654 3210
    ///     NNNN FTBM
    ///     |||| |||+-- Hard-wired nametable mirroring type
    ///     |||| |||     0: Horizontal or mapper-controlled  1: Vertical
    ///     |||| ||+--- "Battery" and other non-volatile memory
    ///     |||| ||      0: Not present  1: Present
    ///     |||| |+--- 512-byte Trainer
    ///     |||| |      0: Not present 1: Present between Header and PRG-ROM data
    ///     |||| +---- Hard-wired four-screen mode
    ///     ||||        0: No 1: Yes
    ///     ++++------ Mapper Number D0..D3
    pub flags_6: u8,
    /// MMMMVVPU
    ///
    /// * M: High nibble of mapper number Mapper 编号高4位
    /// * V: If 0b10, all following flags are in NES 2.0 format
    /// * P: ROM is for the PlayChoice-10
    /// * U: ROM is for VS Unisystem
    pub flags_7: u8,
    ///   D~7654 3210
    ///     SSSS NNNN
    ///     |||| ++++- Mapper number D8..D11
    ///     ++++------ Submapper number
    pub flags_8: u8,
    pub zero: [u8; 7],
}

impl NesHeader {
    /// Return the mapper ID.
    pub fn mapper(&self) -> u16 {
        let ines: u8 = (self.flags_7 & 0xf0) | (self.flags_6 >> 4);
        if self.nes2() {
            ines as u16 | ((self.flags_8 as u16 & 0x0F) << 8)
        } else {
            ines as u16
        }
    }

    pub fn four_screen(&self) -> bool {
        (self.flags_6 & 0b1000) != 0
    }

    pub fn trainer(&self) -> bool {
        (self.flags_6 & 0b0100) != 0
    }

    pub fn save_ram(&self) -> bool {
        (self.flags_6 & 0b0010) != 0
    }
    /// 0: Horizontal or mapper-controlled
    /// 1: Vertical
    pub fn vertical_mirror(&self) -> bool {
        (self.flags_6 & 0b0001) != 0
    }

    pub fn nes2(&self) -> bool {
        self.flags_7 & 0x0C == 0x08
    }
}

impl fmt::Display for NesHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            "
            PRG-ROM: {} KB, CHR-ROM: {} KB
            Mapper ID: {}
            Support Four Screen: {}
            Have Trainer: {}
            Have Save RAM: {}
            Use Vertical Mirror: {}
            ",
            self.prg_rom_size as u32 * 16,
            self.chr_rom_size as u32 * 8,
            self.mapper(),
            self.four_screen(),
            self.trainer(),
            self.save_ram(),
            self.vertical_mirror()
        )
    }
}

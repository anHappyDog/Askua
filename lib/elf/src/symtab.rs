pub struct ElfSym32 {
    pub st_name: u32,
    pub st_value: u32,
    pub st_size: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
}

pub struct ElfSym64 {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

bitflags::bitflags! {
    pub struct ElfSymInfo : u8 {
        const STT_NOTYPE = 0;
        const STT_OBJECT = 1;
        const STT_FUNC = 2;
        const STT_SECTION = 3;
        const STT_FILE = 4;
        const STT_LOPROC = 13;
        const STT_HIPROC = 15;
        const STT_LOCAL = 0;
        const STT_GLOBAL = 1;
        const STT_WEAK = 2;
    }
    pub struct ElfSymOther : u8 {
        const STV_DEFAULT = 0;
        const STV_INTERNAL = 1;
        const STV_HIDDEN = 2;
        const STV_PROTECTED = 3;
    }
}

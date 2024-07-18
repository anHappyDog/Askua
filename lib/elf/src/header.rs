use super::ElfType;

impl ElfType for u32 {}
impl ElfType for u64 {}

bitflags::bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    pub struct ElfHeaderType : u16 {
        const ET_NONE = 0;
        const ET_REL = 1;
        const ET_EXEC = 2;
        const ET_DYN = 3;
        const ET_CORE = 4;
    }
}

impl<T> ElfHeader<T>
where
    T: Clone + Copy + ElfType,
{
    pub fn entry(&self) -> T {
        self.e_entry
    }
}

pub struct ElfIdent {
    ei_mag: [u8; 4],
    ei_class: u8,
    ei_data: u8,
    ei_version: u8,
    ei_osabi: u8,
    ei_abiversion: u8,
    ei_pad: [u8; 7],
}

pub struct ElfHeader<T>
where
    T: Clone + Copy + ElfType,
{
    e_ident: ElfIdent,
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: T,
    e_phoff: T,
    e_shoff: T,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

pub struct ElfProgramHeader<T>
where
    T: Clone + Copy + ElfType,
{
    p_type: u32,
    p_flags: u32,
    p_offset: T,
    p_vaddr: T,
    p_paddr: T,
    p_filesz: T,
    p_memsz: T,
    p_align: T,
}

bitflags::bitflags! {
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    pub struct ElfProgramHeaderFlags: u32 {
        const PF_X = 0x1;
        const PF_W = 0x2;
        const PF_R = 0x4;
    }
    #[derive(PartialEq, Eq, PartialOrd, Ord)]
    pub struct ElfProgramHeaderType : u32 {
        const PT_NULL = 0;
        const PT_LOAD = 1;
        const PT_DYNAMIC = 2;
        const PT_INTERP = 3;
        const PT_NOTE = 4;
        const PT_SHLIB = 5;
        const PT_PHDR = 6;
        const PT_TLS = 7;
        const PT_LOOS = 0x60000000;
        const PT_HIOS = 0x6fffffff;
        const PT_LOPROC = 0x70000000;
        const PT_HIPROC = 0x7fffffff;

    }
}

impl From<u32> for ElfProgramHeaderFlags {
    fn from(flags: u32) -> Self {
        ElfProgramHeaderFlags::from_bits_truncate(flags)
    }
}

impl From<u32> for ElfProgramHeaderType {
    fn from(value: u32) -> Self {
        match value {
            0 => ElfProgramHeaderType::PT_NULL,
            1 => ElfProgramHeaderType::PT_LOAD,
            2 => ElfProgramHeaderType::PT_DYNAMIC,
            3 => ElfProgramHeaderType::PT_INTERP,
            4 => ElfProgramHeaderType::PT_NOTE,
            5 => ElfProgramHeaderType::PT_SHLIB,
            6 => ElfProgramHeaderType::PT_PHDR,
            7 => ElfProgramHeaderType::PT_TLS,
            _ => {
                panic!("Invalid program header type");
            }
        }
    }
}

impl From<ElfProgramHeaderFlags> for u32 {
    fn from(flags: ElfProgramHeaderFlags) -> u32 {
        flags.bits()
    }
}

impl From<ElfProgramHeaderType> for u32 {
    fn from(value: ElfProgramHeaderType) -> u32 {
        value.bits()
    }
}

impl<T> ElfProgramHeader<T>
where
    T: Clone + Copy + ElfType,
{
    pub fn pt_load(&self) -> bool {
        self.p_type == u32::from(ElfProgramHeaderType::PT_LOAD)
    }
    pub fn pt_dynamic(&self) -> bool {
        self.p_type == u32::from(ElfProgramHeaderType::PT_DYNAMIC)
    }
    pub fn pt_interp(&self) -> bool {
        self.p_type == u32::from(ElfProgramHeaderType::PT_INTERP)
    }
    pub fn pt_note(&self) -> bool {
        self.p_type == u32::from(ElfProgramHeaderType::PT_NOTE)
    }
    pub fn pt_shlib(&self) -> bool {
        self.p_type == u32::from(ElfProgramHeaderType::PT_SHLIB)
    }
    pub fn pt_phdr(&self) -> bool {
        self.p_type == u32::from(ElfProgramHeaderType::PT_PHDR)
    }
    pub fn pt_tls(&self) -> bool {
        self.p_type == u32::from(ElfProgramHeaderType::PT_TLS)
    }
}

bitflags::bitflags! {
    pub struct ElfSectionHeaderFlags: u32 {
        const SHF_WRITE = 0x1;
        const SHF_ALLOC = 0x2;
        const SHF_EXECINSTR = 0x4;
        const SHF_MERGE = 0x10;
        const SHF_STRINGS = 0x20;
        const SHF_INFO_LINK = 0x40;
        const SHF_LINK_ORDER = 0x80;
        const SHF_OS_NONCONFORMING = 0x100;
        const SHF_GROUP = 0x200;
        const SHF_TLS = 0x400;
        const SHF_MASKOS = 0x0ff00000;
        const SHF_MASKPROC = 0xf0000000;
    }
    pub struct ElfSectionHeaderType: u32 {
        const SHT_NULL = 0;
        const SHT_PROGBITS = 1;
        const SHT_SYMTAB = 2;
        const SHT_STRTAB = 3;
        const SHT_RELA = 4;
        const SHT_HASH = 5;
        const SHT_DYNAMIC = 6;
        const SHT_NOTE = 7;
        const SHT_NOBITS = 8;
        const SHT_REL = 9;
        const SHT_SHLIB = 10;
        const SHT_DYNSYM = 11;
        const SHT_LOPROC = 0x70000000;
        const SHT_HIPROC = 0x7fffffff;
        const SHT_LOUSER = 0x80000000;
        const SHT_HIUSER = 0xffffffff;
    }
}

pub struct ElfSectionHeader<T: ElfType> {
    sh_name: u32,
    sh_type: u32,
    sh_flags: T,
    sh_addr: T,
    sh_offset: T,
    sh_size: T,
    sh_link: u32,
    sh_info: u32,
    sh_addralign: T,
    sh_entsize: T,
}

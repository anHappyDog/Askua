#![no_std]
#![cfg_attr(debug_assertions, allow(unused))]
pub mod header;
pub mod reloc;
pub mod section;
pub mod symtab;

extern crate alloc;

use core::mem;

use alloc::boxed::Box;
use alloc::vec::Vec;
use header::ElfIdent;
use section::Section;

pub trait ElfType {}

pub struct ElfFile<'a, T>
where
    T: ElfType + Clone + Copy,
{
    header: header::ElfHeader<T>,
    program_headers: Vec<header::ElfProgramHeader<T>>,
    section_headers: Vec<header::ElfSectionHeader<T>>,
    sections: Vec<Box<dyn Section>>,
    data: &'a [u8],
}
pub fn header_ident<'a>(data: &'a [u8]) -> Result<header::ElfIdent, &'static str> {
    if data.len() < mem::size_of::<header::ElfIdent>() {
        return Err("Data is too short to contain an ELF header");
    }
    Ok(ElfIdent::from_bytes_copied(data))
}
impl<'a, T> ElfFile<'a, T>
where
    T: ElfType + Clone + Copy,
{
    pub fn load_from_bytes(data: &'a [u8]) -> Result<ElfFile<T>, &'static str> {
        let header = header::ElfHeader::from_bytes_copied(data);
        // check the type endian and class is ok
        let mut elf_file = ElfFile {
            header,
            program_headers: Vec::new(),
            section_headers: Vec::new(),
            sections: Vec::new(),
            data,
        };
        Ok(elf_file)
    }
}

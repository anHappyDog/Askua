#![no_std]

pub mod header;
pub mod section;
pub mod symtab;
pub mod reloc;

pub trait ElfType {}

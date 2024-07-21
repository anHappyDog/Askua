use core::marker::PhantomData;

use alloc::boxed::Box;

pub trait TableLevel {}

pub struct TableLevel1;

pub struct TableLevel2;

pub struct TableLevel3;

impl TableLevel for TableLevel1 {}
impl TableLevel for TableLevel2 {}
impl TableLevel for TableLevel3 {}

#[derive(Clone, Copy)]
pub struct TableEntry(usize);

impl TableEntry {
    pub fn zero() -> Self {
        Self(0)
    }
}

bitflags::bitflags! {
    pub struct EntryFlags: usize {
        const VALID = 1 << 0;
        const READABLE = 1 << 1;
        const WRITABLE = 1 << 2;
        const EXECUTABLE = 1 << 3;
        const USER = 1 << 4;
        const GLOBAL = 1 << 5;
        const ACCESSED = 1 << 6;
        const DIRTY = 1 << 7;
    }
}

const SATP_BASE_MODE: usize = 0;
const SATP_SV39_MODE: usize = (8 << 60);
const SATP_SV48_MODE: usize = (9 << 60);
const SATP_SV57_MODE: usize = (10 << 60);
const SATP_SV64_MODE: usize = (11 << 60);

const ENTRIES_PER_TABLE: usize = 512;

pub struct PageTable<L: TableLevel> {
    level: PhantomData<L>,
    entries: [TableEntry; ENTRIES_PER_TABLE],
}

impl<L> PageTable<L>
where
    L: TableLevel,
{
    fn new() -> Box<Self> {
        let mut table = Box::new(Self {
            level: PhantomData,
            entries: [TableEntry::zero(); ENTRIES_PER_TABLE],
        });
        todo!()
    }
}

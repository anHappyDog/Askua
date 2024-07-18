use core::marker::PhantomData;

pub trait TableLevel {}

pub struct TableLevel1;

pub struct TableLevel2;

pub struct TableLevel3;

pub struct Entry(usize);

pub struct Table<L: TableLevel> {
    level: PhantomData<L>,
    entries: [Entry; 512],
}

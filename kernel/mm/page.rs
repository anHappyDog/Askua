use core::marker::PhantomData;

pub trait PageSize {}

pub struct Page4K;

pub struct Page2M;

pub struct Page1G;

impl PageSize for Page4K {}

impl PageSize for Page2M {}

impl PageSize for Page1G {}

pub struct Frame<T: PageSize> {
    number: usize,
    size: PhantomData<T>,
}

pub struct Page<T: PageSize> {
    number: usize,
    size: PhantomData<T>,
}


pub const PAGE_SIZE : usize = 4096;
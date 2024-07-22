use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::lock::irq_safe::spin::IrqSafeSpinlock;

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

lazy_static::lazy_static! {
    pub static ref FRAME4K_LIST : IrqSafeSpinlock<Vec<Frame<Page4K>>> = frame4k_init();
}

fn frame4k_init() -> IrqSafeSpinlock<Vec<Frame<Page4K>>> {
    todo!()
}

pub const PAGE_SIZE: usize = 4096;
pub const PAGE_2M_SIZE: usize = 2 * 1024 * 1024;
pub const PAGE_1G_SIZE: usize = 1024 * 1024 * 1024;

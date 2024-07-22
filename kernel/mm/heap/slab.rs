use core::alloc::Layout;

use alloc::sync::Arc;

use crate::lock::irq_safe::spin::IrqSafeSpinlock;

use super::{buddy, Allocator};

// provide
struct SlabSlot {}

pub(crate) struct SlabAllocator {
    start: usize,
    end: usize,
}

impl Allocator for SlabAllocator {}

impl SlabAllocator {
    pub(crate) fn init(start: usize, end: usize) -> Self {
        todo!()
    }
    pub(super) fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }
    pub(super) fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}

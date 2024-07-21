use core::alloc::Layout;

use super::Allocator;

// provide

pub(super) struct SlabAllocator {}

impl Allocator for SlabAllocator {}

impl SlabAllocator {
    pub(super) fn init(start: usize, end: usize) -> Self {
        todo!()
    }
    pub(super) fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }
    pub(super) fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}

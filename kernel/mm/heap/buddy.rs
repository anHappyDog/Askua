// 1 to 16 contiguous-frame allocation

use core::alloc::Layout;

struct BuddyBlock {}

pub(super) struct BuddyAllocator {}

impl BuddyAllocator {
    pub(super) fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }
    pub(super) fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}

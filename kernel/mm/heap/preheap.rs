use core::alloc::{GlobalAlloc, Layout};

use crate::driver::plic::sifive;

const PREHEAP_SIZE: usize = 0x1000 * 16;

#[repr(align(4096))]
/// Acutally I have to say that this is used for
/// the fdt and some other kernel obj's allocation,
/// which will never be freed, which means their
/// lifetime is quite static.
pub(super) struct PreHeapPolicy {
    data: [u8; PREHEAP_SIZE],
    current: usize,
}

impl super::Allocator for PreHeapPolicy {}

impl PreHeapPolicy {
    pub const fn new() -> Self {
        Self {
            data: [0; PREHEAP_SIZE],
            current: 0,
        }
    }
    pub(super) fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();
        let current = (self.current + align - 1) & !(align - 1);
        let next = current + size;
        if next > PREHEAP_SIZE {
            panic!("PreHeap OOM");
        }
        self.current = next;
        &mut self.data[current] as *mut u8
    }
}

extern crate alloc;

mod buddy;
mod preheap;
mod slab;

use core::alloc::{GlobalAlloc, Layout};

use preheap::PreHeapPolicy;

use crate::lock::{irq_safe::spin::IrqSafeSpinlock, spin::Spinlock};

pub(self) trait Allocator {}

pub enum HeapPolicy {
    Pre(Spinlock<PreHeapPolicy>),
    Normal(NormalHeapPolicy),
}

impl HeapPolicy {
    pub(self) fn alloc(&self, layout: Layout) -> *mut u8 {
        match self {
            Self::Pre(pre) => pre.lock().alloc(layout),
            Self::Normal(normal) => normal.alloc(layout),
        }
    }
    pub(self) fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        match self {
            Self::Pre(_) => {}
            Self::Normal(normal_heap) => normal_heap.dealloc(ptr, layout),
        }
    }
}

pub(super) struct NormalHeapPolicy {
    slab_allocator: IrqSafeSpinlock<slab::SlabAllocator>,
    buddy_allocator: IrqSafeSpinlock<buddy::BuddyAllocator>,
}

impl NormalHeapPolicy {
    fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() <= 4096 {
            self.slab_allocator.lock().alloc(layout)
        } else {
            self.buddy_allocator.lock().alloc(layout)
        }
    }
    fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if layout.size() <= 4096 {
            self.slab_allocator.lock().dealloc(ptr, layout)
        } else {
            self.buddy_allocator.lock().dealloc(ptr, layout)
        }
    }
}

// the policy will only be changed when the memory map is ok and the normal heap is ready
// so we don't need to add lock to the policy, as it will cause some needless overhead
// if we want to add more policies, we may add lock to the policy
pub struct Heap {
    policy: HeapPolicy,
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.policy.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.policy.dealloc(ptr, layout)
    }
}

impl Heap {
    pub fn change_policy(&mut self, policy: HeapPolicy) {
        self.policy = policy;
    }
    pub const fn new() -> Self {
        Self {
            policy: HeapPolicy::Pre(Spinlock::new(PreHeapPolicy::new())),
        }
    }
}

#[global_allocator]
static HEAP: Heap = Heap::new();

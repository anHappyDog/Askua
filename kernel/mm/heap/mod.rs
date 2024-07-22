extern crate alloc;

pub(super) mod buddy;
mod preheap;
pub(super) mod slab;

use core::alloc::{GlobalAlloc, Layout};

use alloc::{boxed::Box, sync::Arc};
use buddy::BuddyAllocator;
use preheap::PreHeapPolicy;
use slab::SlabAllocator;

use crate::lock::{irq_safe::spin::IrqSafeSpinlock, spin::Spinlock};

use super::{
    page::PAGE_SIZE,
    table::{self, TableLevel1},
};

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
    slab_allocator: Arc<IrqSafeSpinlock<slab::SlabAllocator>>,
    buddy_allocator: Arc<IrqSafeSpinlock<buddy::BuddyAllocator>>,
}

impl NormalHeapPolicy {
    pub(crate) fn new(
        slab_allocator: Arc<IrqSafeSpinlock<SlabAllocator>>,
        buddy_allocator: Arc<IrqSafeSpinlock<BuddyAllocator>>,
    ) -> Self {
        Self {
            slab_allocator,
            buddy_allocator,
        }
    }
    fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() <= PAGE_SIZE {
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
    policy: IrqSafeSpinlock<HeapPolicy>,
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.policy.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.policy.lock().dealloc(ptr, layout)
    }
}

impl Heap {
    #[inline(always)]
    pub fn change_policy(&self, policy: Box<HeapPolicy>) {
        let mut locked_policy = self.policy.lock();
        *locked_policy = *policy;
    }
    pub const fn new() -> Self {
        Self {
            policy: IrqSafeSpinlock::new(HeapPolicy::Pre(Spinlock::new(PreHeapPolicy::new()))),
        }
    }
}

#[global_allocator]
static HEAP: Heap = Heap::new();

pub(super) fn change_policy(policy: Box<HeapPolicy>) {
    HEAP.change_policy(policy);
}

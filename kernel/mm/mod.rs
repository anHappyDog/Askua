use alloc::{boxed::Box, sync::Arc};
use heap::buddy;
use table::TableLevel1;

use crate::lock::irq_safe::spin::IrqSafeSpinlock;

pub mod address;
pub mod page;
pub mod table;

mod heap;

pub fn init()  {
    // create the frame list,which is used for the allocator's dealloc and alloc
    // create the normal heap policy and change
    // the map the kenerl heap and return the PageTable.
    let slab_allocator = Arc::new(IrqSafeSpinlock::new(heap::slab::SlabAllocator::init(0, 0)));
    let buddy_allocator = Arc::new(IrqSafeSpinlock::new(buddy::BuddyAllocator::init(0, 0)));
    let normal_policy = Box::new(heap::HeapPolicy::Normal(heap::NormalHeapPolicy::new(
        slab_allocator,
        buddy_allocator,
    )));
    heap::change_policy(normal_policy);
}

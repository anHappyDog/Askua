// 1 to 16 contiguous-frame allocation

use core::alloc::Layout;

use alloc::vec::Vec;

// order 0: 1 frame
const MAX_ORDER: usize = 10;

struct BuddySlot {
    order: usize,
    addr: usize,
    next: Option<&'static BuddySlot>,
}

struct BuddySlotList {
    slots: Vec<BuddySlot>,
    bitmap: Vec<u8>,
}

impl BuddySlotList {
     fn new() -> Self {
        let mut slots = Vec::new();
        let mut bitmap = Vec::new();
        for _ in 0..MAX_ORDER {
            slots.push(BuddySlot {
                order: 0,
                addr: 0,
                next: None,
            });
            bitmap.push(0);
        }
        Self { slots, bitmap }
    }
    fn alloc_slot(&mut self, order: usize) -> Option<&'static BuddySlot> {
        todo!()
    }
    fn dealloc_slot(&mut self, slot: &'static BuddySlot) {
        todo!()
    }
}

pub(crate) struct BuddyAllocator {
    start: usize,
    end: usize,
    slots: [Option<&'static BuddySlot>; MAX_ORDER],
    slot_list: BuddySlotList,
}

impl BuddyAllocator {
    pub(super) fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }
    pub(super) fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
    pub(crate) fn init(start: usize, end: usize) -> Self {
        todo!()
    }
}

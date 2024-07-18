#![no_std]

extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};

pub trait Allocator {}

pub struct Heap {}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unimplemented!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unimplemented!()
    }
}

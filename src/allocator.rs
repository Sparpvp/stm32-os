pub mod memory;
pub mod symbols;

use core::alloc::GlobalAlloc;

use memory::{free, zalloc_block};

struct KernelGlobalAllocator {}

unsafe impl GlobalAlloc for KernelGlobalAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        zalloc_block(layout.size() as u16)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        free(ptr);
    }
}

#[global_allocator]
static GA: KernelGlobalAllocator = KernelGlobalAllocator {};

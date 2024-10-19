use core::{marker::PhantomData, mem::size_of, ptr::null_mut};

// use cortex_m_semihosting::hprintln;
use crate::allocator::symbols;
use symbols::*;

#[repr(C)]
pub struct FreeList {
    pub size: u16,
    pub memory: *mut u8,
    _padding: PhantomData<[u8; 2]>, // 2 bytes padding required for 4-byte alignment
}

struct FreeListWrapper(*mut FreeList);
unsafe impl Sync for FreeListWrapper {}

static mut FREE_LIST: FreeListWrapper = FreeListWrapper(0 as *mut FreeList);

impl FreeList {
    pub fn init() {
        assert_eq!(unsafe { FREE_LIST.0 }, null_mut());

        let heap_start = get_heap_start();
        let heap = heap_start as *mut Self;
        unsafe {
            (*heap).size = get_heap_size() as u16;
            (*heap).memory = 0 as *mut u8;

            FREE_LIST = FreeListWrapper(heap);
        };
    }
}

#[inline]
fn assert_4_alignment(block: *mut u8) {
    assert_eq!(block as usize % 4 == 0, true);
}

unsafe fn init_metadata(heap: *mut FreeList, size: u16, is_allocated: bool) {
    assert_4_alignment(heap as *mut u8);

    (*heap).size = size;
    if is_allocated {
        (*heap).size |= 1;
    } else {
        (*heap).size &= !1;
    }
}

unsafe fn zero_memory(block: *mut u8, size: u16) {
    assert_4_alignment(block);
    block.write_bytes(0, size as usize);
}

unsafe fn alloc_first_fit(heap: *mut FreeList, size: u16) -> *mut FreeList {
    assert_4_alignment(heap as *mut u8);
    assert_eq!(size < get_heap_size() as u16, true);

    let mut tmp_head = heap;
    let heap_end = get_heap_end();
    while (tmp_head as usize) < heap_end && (((*tmp_head).size & 1) == 1 || (*tmp_head).size < size)
    {
        tmp_head = tmp_head.byte_add((((*tmp_head).size & (!1)) as usize) + size_of::<FreeList>());
    }
    if (tmp_head as usize) >= heap_end {
        return 0 as *mut FreeList; // Out of memory
    }

    if (*tmp_head).size == size {
        init_metadata(tmp_head, 0, true);
    } else if (*tmp_head).size > size {
        let remaining_size = (*tmp_head).size - size;
        init_metadata(tmp_head, size, true);
        init_metadata(
            tmp_head.byte_add(size as usize + size_of::<FreeList>()),
            remaining_size,
            false,
        );
    }

    // TODO: Consider returning just the memory block not freelist
    tmp_head
}

pub fn free(ptr: *mut u8) {
    assert_4_alignment(ptr);

    let block = unsafe { ptr.byte_sub(size_of::<FreeList>()) } as *mut FreeList;
    unsafe {
        zero_memory((*block).memory, (*block).size);
        init_metadata(block, (*block).size, false);
    };

    // TODO: coalesce blocks
}

pub fn zalloc_block(size: u16) -> *mut u8 {
    let size = (size + 3) & (!3); // Alignes to 4 bytes

    unsafe {
        match FREE_LIST {
            FreeListWrapper(heap) if heap == 0 as *mut FreeList => {
                return 0 as *mut u8;
            }
            // non-NULL heap: it has been initialized
            FreeListWrapper(heap) => {
                // hprintln!("Allocating size {}...", size).unwrap();
                let newblock_ptr = alloc_first_fit(heap, size);
                if newblock_ptr == 0 as *mut FreeList {
                    return 0 as *mut u8;
                }
                (*newblock_ptr).memory = (newblock_ptr.byte_add(size_of::<FreeList>())) as *mut u8;

                zero_memory((*newblock_ptr).memory, size);
                return (*newblock_ptr).memory;
            }
        }
    }
}

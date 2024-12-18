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
#[inline]
fn assert_8_alignment(block: *mut u8) {
    assert_eq!(block as usize % 8 == 0, true);
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

    let size_in_between: i16 =
        (*tmp_head).size as i16 - 2 * size_of::<FreeList>() as i16 - size as i16;

    if (*tmp_head).size == size {
        init_metadata(tmp_head, size, true);
    } else if size_in_between < 4 {
        init_metadata(tmp_head, (*tmp_head).size, true);
    } else if (*tmp_head).size > size {
        let mut remaining_size = (*tmp_head).size - size;

        // We have to be careful in initializing the size of the next block
        //  as if we are in between already allocated blocks we could erroneously overwrite them.
        if (tmp_head.byte_add((*tmp_head).size as usize + size_of::<FreeList>()) as usize)
            < heap_end
        {
            // We are indeed in between some blocks
            remaining_size = size_in_between as u16;
        }
        init_metadata(tmp_head, size, true);
        init_metadata(
            tmp_head.byte_add(size as usize + size_of::<FreeList>()),
            remaining_size,
            false,
        );
    }

    tmp_head
}

fn pad_at(block: *mut FreeList, size: u16) {
    unsafe {
        let size_in_between: i16 =
            (*block).size as i16 - 2 * size_of::<FreeList>() as i16 - size as i16;
        let mut remaining_size = (*block).size - size;

        if (block.byte_add((*block).size as usize + size_of::<FreeList>()) as usize)
            < get_heap_end()
        {
            // We are indeed in between some blocks
            remaining_size = size_in_between as u16;
        }

        init_metadata(block, size, true);
        init_metadata(
            block.byte_add(size as usize + size_of::<FreeList>()),
            remaining_size,
            false,
        );
    }
}

fn first_stackable_block(heap: *mut FreeList, size: u16) -> *mut FreeList {
    unsafe {
        let mut tmp_head = heap;
        let heap_end = get_heap_end();

        /*
            We search for size >= size IF it's already aligned (hence we can use the block)
            OR for size >= size+4+2*8 (2*header + 4 alloc)
        */
        while (tmp_head as usize) < heap_end
            && (((*tmp_head).size & 1) == 1
                || ((*tmp_head).size < size && (*tmp_head).size % 8 == 0)
                || ((*tmp_head).size < size + 4 + 2 * size_of::<FreeList>() as u16
                    && (*tmp_head).size % 8 == 4))
        {
            tmp_head =
                tmp_head.byte_add((((*tmp_head).size & (!1)) as usize) + size_of::<FreeList>());
        }

        if (tmp_head as usize) >= heap_end {
            return 0 as *mut FreeList; // Out of memory
        }

        return tmp_head;
    }
}

pub fn free(ptr: *mut u8) {
    assert_4_alignment(ptr);

    let block = unsafe { ptr.byte_sub(size_of::<FreeList>()) } as *mut FreeList;
    // & (!1) since we want to discard last bit
    let old_size: u16 = unsafe { (*block).size } & (!1);
    unsafe {
        zero_memory((*block).memory, old_size);
        init_metadata(block, old_size, false);
    };

    // TODO: coalesce blocks
}

pub fn free_in_range(ptr: *mut u8) {
    assert_4_alignment(ptr);

    let mut heap_head = unsafe {
        match FREE_LIST {
            FreeListWrapper(heap) if heap == 0 as *mut FreeList => {
                panic!("Uninitialized heap in free_in_range.");
            }
            FreeListWrapper(heap) => heap,
        }
    };

    let heap_end = get_heap_end();

    unsafe {
        let mut prec = heap_head;

        while (heap_head as usize) < heap_end
            && (((*heap_head).size & 1) == 1)
            && heap_head < ptr as *mut FreeList
        {
            prec = heap_head;
            heap_head =
                heap_head.byte_add((((*heap_head).size & (!1)) as usize) + size_of::<FreeList>());
        }

        if heap_head == ptr as *mut FreeList {
            free(heap_head as *mut u8);
        } else {
            free(prec as *mut u8);
        }
    };
}

pub fn zalloc_stack(size: u16) -> *mut u8 {
    let size = (size + 7) & (!7); // Alignes to 8 bytes
    unsafe {
        match FREE_LIST {
            FreeListWrapper(heap) if heap == 0 as *mut FreeList => {
                return 0 as *mut u8;
            }
            FreeListWrapper(heap) => {
                let last_block = first_stackable_block(heap, size);
                if last_block as usize % 8 == 4 {
                    // Quick & dirty trick to align to 8 bytes.
                    // Actually occupies 12 bytes since the header is 8 bytes
                    pad_at(last_block, 4);
                }

                let ret_stack = zalloc_block(size);
                assert_8_alignment(ret_stack);
                ret_stack
            }
        }
    }
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
                    panic!("Out of memory!");
                }
                (*newblock_ptr).memory = (newblock_ptr.byte_add(size_of::<FreeList>())) as *mut u8;

                zero_memory((*newblock_ptr).memory, size);
                return (*newblock_ptr).memory;
            }
        }
    }
}

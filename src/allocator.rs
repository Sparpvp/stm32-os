use core::{marker::PhantomData, mem::size_of};

// use cortex_m_semihosting::hprintln;

extern "C" {
    static _heap_start: usize;
    static _heap_size: usize;
    static _memory_end: usize;
}

pub fn get_heap_start() -> usize {
    let hstart = unsafe { &_heap_start as *const usize } as usize;
    hstart
}

pub fn get_heap_size() -> usize {
    let hsize = unsafe { &_heap_size as *const usize } as usize;
    hsize
}

pub fn get_memory_end() -> usize {
    let mend = unsafe { &_memory_end as *const usize } as usize;
    mend
}

pub fn get_heap_end() -> usize {
    get_heap_start() + get_heap_size()
}

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
        let heap_start = get_heap_start();
        let heap = heap_start as *mut Self;
        unsafe {
            (*heap).size = get_heap_size() as u16;
            (*heap).memory = 0 as *mut u8;

            FREE_LIST = FreeListWrapper(heap);
        };
    }
}

unsafe fn init_metadata(heap: *mut FreeList, size: u16, is_allocated: bool) {
    (*heap).size = size;
    if is_allocated {
        (*heap).size |= 1;
    } else {
        (*heap).size &= !1;
    }
}

unsafe fn zero_memory(block: *mut u8, size: u16) {
    block.write_bytes(0, size as usize);
}

unsafe fn alloc_first_fit(heap: *mut FreeList, size: u16) -> *mut FreeList {
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

pub unsafe fn zalloc_block(size: u16) -> *mut u8 {
    let size = (size + 3) & (!3); // Alignes to 4 bytes

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

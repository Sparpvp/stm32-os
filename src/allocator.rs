use core::mem::size_of;

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

unsafe fn zero_memory(block: *mut FreeList, size: u16) {
    let mut tmp_block = block as *mut u8;
    tmp_block = tmp_block.add(size_of::<u16>());
    for i in 0..size {
        *(tmp_block.add(i as usize)) = 0;
    }
}

unsafe fn alloc_first_fit(heap: *mut FreeList, size: u16) -> *mut FreeList {
    assert_eq!(size < get_heap_size() as u16, true);

    let mut tmp_head = heap;
    let heap_end = get_heap_end();
    while (tmp_head as usize) < heap_end && ((*tmp_head).size & 1) == 0 && (*tmp_head).size < size {
        if (tmp_head as usize) >= heap_end {
            return 0 as *mut FreeList; // Out of memory
        }
        tmp_head = tmp_head.add(((*tmp_head).size & (!1)) as usize);
    }

    if (*tmp_head).size == size {
        init_metadata(tmp_head, 0, true);
    } else if (*tmp_head).size > size {
        let remaining_size = (*tmp_head).size - size;
        init_metadata(tmp_head, size, true);
        init_metadata(
            tmp_head.add(size as usize + size_of::<u16>()),
            remaining_size,
            false,
        );
    }

    tmp_head
}

pub unsafe fn zalloc_block(size: u16) -> *mut FreeList {
    // TODO: Align to 4 bytes if necessary.
    let size = ((size + 1) >> 1) << 1; // Makes size even.

    match FREE_LIST {
        FreeListWrapper(heap) if heap == 0 as *mut FreeList => {
            return 0 as *mut FreeList;
        }
        // non-NULL heap: it has been initialized
        FreeListWrapper(heap) => {
            let newblock_ptr = alloc_first_fit(heap, size);
            if newblock_ptr == 0 as *mut FreeList {
                return 0 as *mut FreeList;
            }

            zero_memory(newblock_ptr, size);
            return newblock_ptr;
        }
    }
}

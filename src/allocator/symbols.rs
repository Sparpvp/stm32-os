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

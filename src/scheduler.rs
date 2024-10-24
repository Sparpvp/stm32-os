use core::{
    mem::{self, size_of, MaybeUninit},
    ptr::{self, null_mut},
};

use crate::{allocator::memory::zalloc_block, process::Process};

#[repr(C)]
pub struct ScheduleList {
    pub proc: MaybeUninit<Process>,
    pub next: *mut ScheduleList,
}

pub struct Scheduler(pub *mut ScheduleList);
pub static mut PROC_LIST: Scheduler = Scheduler(0 as *mut ScheduleList);
#[no_mangle]
#[used]
pub static mut CURR_PROC: MaybeUninit<ScheduleList> = MaybeUninit::zeroed();

impl Scheduler {
    pub fn init() {
        assert_eq!(unsafe { PROC_LIST.0 }, null_mut());

        let head = zalloc_block(size_of::<ScheduleList>() as u16) as *mut ScheduleList;
        let null_schedule = ScheduleList {
            proc: MaybeUninit::zeroed(),
            next: null_mut(),
        };
        unsafe {
            // Initialize without dropping
            // We don't have anything to drop since the value is 0
            // It'd try to free a null pointer otherwise.
            head.write(null_schedule);
            PROC_LIST = Scheduler(head);
        }
    }

    #[no_mangle]
    pub unsafe fn next_proc() {
        let proc = CURR_PROC.assume_init_mut();

        if proc.next.is_null() {
            let read = ptr::read(PROC_LIST.0);
            let _ = mem::replace(proc, ptr::read(PROC_LIST.0));
        } else {
            let next_proc = ptr::read(proc.next);
            let _ = mem::replace(proc, next_proc);
        }
    }
}

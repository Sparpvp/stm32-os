use core::{
    mem::{size_of, MaybeUninit},
    ptr::null_mut,
};

use crate::{allocator::memory::zalloc_block, process::Process};

pub struct ScheduleList {
    pub proc: MaybeUninit<Process>,
    pub next: *mut ScheduleList,
}

pub struct Scheduler(pub *mut ScheduleList);
pub static mut PROC_LIST: Scheduler = Scheduler(0 as *mut ScheduleList);
pub static mut CURR_PROC: *mut ScheduleList = null_mut();

impl Scheduler {
    pub fn init() {
        assert_eq!(unsafe { PROC_LIST.0 }, null_mut());

        let head = zalloc_block(size_of::<ScheduleList>() as u16) as *mut ScheduleList;
        let null_schedule = ScheduleList {
            proc: MaybeUninit::uninit(),
            next: null_mut(),
        };
        unsafe {
            // Initialize without dropping
            // We don't have anything to drop since the value is 0
            // It'd try to free a null pointer otherwise.
            head.write(null_schedule);
            CURR_PROC = head;
            PROC_LIST = Scheduler(head);
        }
    }

    pub unsafe fn next_proc() {
        if (*CURR_PROC).next == null_mut() {
            CURR_PROC = PROC_LIST.0; // Reset to the head
        } else {
            CURR_PROC = (*CURR_PROC).next;
        }
    }
}

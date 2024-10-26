use core::{
    mem::{self, MaybeUninit},
    ptr::{self, null_mut},
};

use crate::{process::Process, trap::FIRST_CTX_SWITCH};

#[repr(C)]
pub struct ScheduleList {
    pub proc: MaybeUninit<Process>,
    pub next: *mut ScheduleList,
}

pub struct Scheduler(pub *mut ScheduleList);
pub static mut PROC_LIST: Scheduler = Scheduler(0 as *mut ScheduleList);
#[no_mangle]
#[used]
pub static mut CURR_PROC: MaybeUninit<ScheduleList> = MaybeUninit::uninit();

impl Scheduler {
    #[no_mangle]
    pub unsafe fn next_proc() {
        let proc = CURR_PROC.assume_init_mut();

        if FIRST_CTX_SWITCH || proc.next == null_mut() {
            // Put the head as the new process
            let _ = mem::replace(proc, ptr::read(PROC_LIST.0));
        } else {
            // Switch to next process since there's one.
            let next_proc = ptr::read(proc.next);
            let _ = mem::replace(proc, next_proc);
        }
    }
}

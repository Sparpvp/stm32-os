use core::{
    mem::{self, MaybeUninit},
    ptr::{self, null_mut},
};

use crate::{
    process::{Process, ProcessState},
    trap::FIRST_CTX_SWITCH,
};

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
        let curr_proc = CURR_PROC.assume_init_mut();
        curr_proc.proc.assume_init_mut().state = ProcessState::Ready;

        let mut next_proc: ScheduleList;
        if FIRST_CTX_SWITCH || curr_proc.next == null_mut() {
            // Put the head as the new process
            next_proc = ptr::read(PROC_LIST.0);
        } else {
            // Switch to next process since there's one.
            next_proc = ptr::read(curr_proc.next);
        }

        next_proc.proc.assume_init_mut().state = ProcessState::Running;
        let _ = mem::replace(curr_proc, next_proc);
    }
}

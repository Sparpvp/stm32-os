use core::{
    mem::{self, MaybeUninit},
    ptr::{self, null_mut},
};

use crate::{
    process::{Process, ProcessState},
    trap::FIRST_CTX_SWITCH,
};

extern "C" {
    fn _switch_to_psp(psp: *mut u8);
}

#[repr(C)]
pub struct ScheduleList {
    pub proc: MaybeUninit<Process>,
    pub next: *mut ScheduleList,
}

pub struct Scheduler {
    pub head: *mut ScheduleList,    // Head might be repositioned
    pub current: *mut ScheduleList, // Current will be modified at each context switch
}

#[no_mangle]
#[used]
pub static mut PROC_LIST: Option<Scheduler> = None;
#[no_mangle]
#[used]
pub static mut CURR_PROC: MaybeUninit<ScheduleList> = MaybeUninit::uninit();

impl Scheduler {
    // Inlining is mandatory here since the stack changes
    //  and the epilogue would break everything
    #[inline(always)]
    pub fn init(psp: *mut u8) {
        unsafe {
            // Set PSP as default stack, flush the pipeline
            //  standard procedure. Using barriers.
            _switch_to_psp(psp);
        }
    }

    // Safety: Assumes PROC_LIST and CURR_LIST are initialized.
    // As long as at least one .new() was invoked on the spawner, the constraint holds.
    #[no_mangle]
    pub unsafe fn next_proc() {
        let mut process_list = PROC_LIST.take().unwrap();
        let curr_proc = CURR_PROC.assume_init_mut();
        curr_proc.proc.assume_init_mut().state = ProcessState::Ready;

        // Save the current PCB into the list
        // Not dropping the previous process inside the MaybeUninit not only is not an issue, it is mandatory.
        // That's because the process that's being overwritten is the same and effectively uses
        //  the same heap-allocated block.
        (*process_list.current)
            .proc
            .write(ptr::read(curr_proc.proc.assume_init_ref()));

        let mut next_proc: ScheduleList;
        if FIRST_CTX_SWITCH || (*process_list.current).next == null_mut() {
            // Put the head as the new process to be scheduled
            let head = ptr::read(process_list.head);
            next_proc = head;

            process_list.current = process_list.head;
        } else {
            // Switch to the next process since there's one.
            let next = (*process_list.current).next;
            next_proc = ptr::read(next);

            process_list.current = (*process_list.current).next;
        }

        next_proc.proc.assume_init_mut().state = ProcessState::Running;
        let _ = mem::replace(curr_proc, next_proc);

        PROC_LIST.replace(process_list);
    }
}

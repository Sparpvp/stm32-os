use core::ptr::null_mut;

use crate::{allocator::memory::zalloc_block, process::Process};

struct ScheduleList {
    proc: Option<Process>,
    next: *mut ScheduleList,
}

// Dovrebbe tenere un puntatore o uno stack allocated?
struct ScheduleWrapper(*mut ScheduleList);
static mut PROC_LIST: ScheduleWrapper = ScheduleWrapper(0 as *mut ScheduleList);

impl ScheduleWrapper {
    fn init() {
        let head = zalloc_block(size_of::<ScheduleList>() as u16) as *mut ScheduleList;
        let null_schedule = ScheduleList {
            proc: Option::None,
            next: null_mut(),
        };
        unsafe {
            *head = null_schedule;
            PROC_LIST = ScheduleWrapper(head);
        }
    }

    // prob non funziona niente lol
    fn schedule_next(&mut self, proc: Process) {
        let mut head = unsafe { &mut *(self.0) };
        while head.proc.is_some() {
            head = unsafe { &mut *(head.next) };
        }
        let new_contact =
            unsafe { &mut *(zalloc_block(size_of::<ScheduleList>() as u16) as *mut ScheduleList) };
        new_contact.proc = Some(proc);
        new_contact.next = null_mut();
    }
}

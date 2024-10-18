use core::{mem::size_of, ptr::null_mut};

use crate::{allocator::memory::zalloc_block, process::Process};

pub struct ScheduleList {
    pub proc: Option<Process>,
    pub next: *mut ScheduleList,
}

pub struct ProcListWrapper(pub *mut ScheduleList);
pub static mut PROC_LIST: ProcListWrapper = ProcListWrapper(0 as *mut ScheduleList);

impl ProcListWrapper {
    pub fn init() {
        assert_eq!(unsafe { PROC_LIST.0 } == 0 as *mut ScheduleList, true);

        let head = zalloc_block(size_of::<ScheduleList>() as u16) as *mut ScheduleList;
        let null_schedule = ScheduleList {
            proc: Option::None,
            next: null_mut(),
        };
        unsafe {
            // Initialize without dropping
            // We don't have anything to drop since the value is 0
            // It'd try to free a null pointer otherwise.
            head.write(null_schedule);
            PROC_LIST = ProcListWrapper(head);
        }
    }
}

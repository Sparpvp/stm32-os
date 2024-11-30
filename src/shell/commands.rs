use core::{
    mem::transmute,
    ptr::{self, null_mut},
};

use alloc::string::String;

use crate::{
    dispatcher::ProcessIdentifier,
    process::{Process, STACK_SIZE},
    scheduler::PROC_LIST,
    trap::critical_section::CriticalSection,
};

use super::ShellError;

pub(in crate::shell) fn rm_proc_by_pid(pid: u16, _cs: &CriticalSection) -> Result<(), ShellError> {
    let p = unsafe { PROC_LIST.as_mut().unwrap() };
    let mut ptr_tmp = p.head;
    let mut prec = ptr_tmp;

    let res = unsafe {
        while !ptr_tmp.is_null() && (*ptr_tmp).proc.assume_init_ref().pid != pid {
            prec = ptr_tmp;
            ptr_tmp = (*ptr_tmp).next;
        }

        if ptr_tmp.is_null() || ptr_tmp == p.head {
            Err(ShellError::ExecutionError)
        } else {
            if (*ptr_tmp).next.is_null() {
                // If I'm deleting at the end
                (*prec).next = null_mut();
                let curr_proc = (*prec).proc.assume_init_mut() as *mut _;
                drop(ptr::read(curr_proc));
            } else if ptr_tmp != p.head {
                // In-Between elimination
                (*prec).next = (*ptr_tmp).next;
                let curr_proc = (*ptr_tmp).proc.assume_init_mut() as *mut _;
                drop(ptr::read(curr_proc));
            }

            Ok(())
        }
    };

    res
}

pub(in crate::shell) fn rm_proc_by_name(
    name: String,
    _cs: &CriticalSection,
) -> Result<(), ShellError> {
    Ok(())
}

pub(in crate::shell) fn add_proc(
    _cs: &CriticalSection,
    proc_name: &str,
) -> Result<(), ShellError> {
    let func = ProcessIdentifier::retrieve_base_address(proc_name)
        .ok_or_else(|| ShellError::ExecutionError)?;
    let func: fn() = unsafe { transmute(func) };

    Process::new(func, STACK_SIZE).enqueue();

    Ok(())
}

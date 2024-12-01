use core::arch::asm;

use crate::trap::pend_sv_set;

#[repr(u8)]
pub enum SVCallId {
    Yield = 1,
}

pub fn handle_syscall(comment: u32, ret_pc: &mut u32) {
    // Here are defined what the syscall ids will be
    match comment {
        1 => {
            // Yield
            // We set PendSV to activate the next context switch.
            // In this way the next process is automatically scheduled with no external logic
            pend_sv_set();
        }
        _ => panic!("Unrecognized syscall number!\n"),
    };
}

pub unsafe fn syscall(id: SVCallId) {
    asm!("SVC #0", in("r0") id as u8);
}

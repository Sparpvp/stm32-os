use core::arch::asm;

pub struct CriticalSection;
impl CriticalSection {
    pub(in crate::trap::critical_section) fn new() -> CriticalSection {
        CriticalSection
    }
}

// Execute function func in an environment with PRIMASK enabled
// The function func cannot be interrupted.
pub fn critical_section<F, T>(func: F) -> T
where
    F: FnOnce(&CriticalSection) -> T,
{
    let primask_status: u8;
    unsafe {
        asm!("MRS {}, PRIMASK", out(reg) primask_status);
    };

    unsafe {
        asm!("CPSID i");
    };
    let ret = func(&CriticalSection::new());
    // If PRIMASK was already enabled before, DON'T disactivate it!
    if primask_status == 0 {
        unsafe {
            asm!("CPSIE i");
        };
    }

    ret
}

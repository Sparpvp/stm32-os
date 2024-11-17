use core::arch::asm;

pub fn critical_section<F, T>(func: F) -> T
where
    F: Fn() -> T,
{
    unsafe {
        asm!("CPSID i");
    };
    let ret = func();
    unsafe {
        asm!("CPSIE i");
    };

    ret
}

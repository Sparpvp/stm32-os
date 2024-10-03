#![no_std]
#![no_main]

pub mod panic;

use cortex_m_semihosting::hprintln;

#[no_mangle]
extern "C" fn kmain() -> ! {
    hprintln!("Xemo vivi!").unwrap();
    loop {}
}

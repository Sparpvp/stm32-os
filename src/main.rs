#![no_std]
#![no_main]

pub mod panic;
pub mod peripherals;

use cortex_m_semihosting::hprintln;

#[no_mangle]
extern "C" fn kmain() -> ! {
    hprintln!("Xemo vivi!").unwrap();

    loop {}
}

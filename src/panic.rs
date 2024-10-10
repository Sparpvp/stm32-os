use core::arch::asm;
use cortex_m_semihosting::hprintln;

#[no_mangle]
extern "C" fn eh_personality() {}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    hprintln!("Panicking...: ").unwrap();

    match info.location() {
        Some(l) => {
            hprintln!("line {}, file {}", l.line(), l.file()).unwrap();
        }
        None => {
            hprintln!("unable to retrieve panic info").unwrap();
            unreachable!()
        }
    }

    // Call the abort handler
    abort();
}

#[no_mangle]
extern "C" fn abort() -> ! {
    loop {
        unsafe { asm!("wfi") }
    }
}

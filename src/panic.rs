use core::arch::asm;

#[no_mangle]
extern "C" fn eh_personality() {}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // TODO: USART print

    match info.location() {
        Some(l) => {
            // TODO USART print
        }
        None => {
            // TODO USART print
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

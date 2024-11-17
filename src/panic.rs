use core::arch::asm;

use alloc::string::String;

#[macro_export]
macro_rules! print {
    ($($args:tt)+) => {{
        use crate::peripherals::usart;
        use core::fmt::Write;
        unsafe {
            let usart = usart::G_USART.as_mut();
            if usart.is_none() {
                abort();
            }
            let usart = usart.unwrap();
            let _ = write!(usart, $($args)+);
        }
    }};
}

#[macro_export]
macro_rules! println
{
	() => ({
		print!("\r\n")
	});
	($fmt:expr) => ({
		print!(concat!($fmt, "\r\n"))
	});
	($fmt:expr, $($args:tt)+) => ({
		print!(concat!($fmt, "\r\n"), $($args)+)
	});
}

#[no_mangle]
extern "C" fn eh_personality() {}
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        asm!("CPSID i");
    };

    match info.location() {
        Some(l) => {
            println!(
                "An unrecoverable runtime error occured in file `{}`, at line: {}. Payload: {}.",
                l.file(),
                l.line(),
                info
            );
        }
        None => {
            unreachable!("This case should be unreachable: no panic location has been identified.")
        }
    }

    // Since we are in an unrecoverable state, call the abort handler
    abort();
}

#[no_mangle]
extern "C" fn abort() -> ! {
    loop {
        unsafe { asm!("wfi") }
    }
}

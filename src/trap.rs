use core::{arch::asm, ptr::read_volatile};

const ICSR_ADDR: u32 = 0xE000ED04;

#[no_mangle]
extern "C" fn rust_trap_handler() {
    // Save current registers onto the stack
    unsafe {
        asm!("PUSH {{r0-r7}}");
    };

    let mut return_pc: u16;
    unsafe {
        asm!("mov {}, pc", out(reg) return_pc);
    };

    // Match on the Interrupt Control and State Register (ICSR)
    let icsr: u32 = unsafe { read_volatile(ICSR_ADDR as *const u32) };
    let exception_number: u8 = (icsr & 0b111111) as u8; // VECTACTIVE register

    match exception_number {
        2 => {
            // NMI
            panic!("Non-Maskable Interrupt triggered!\n");
        }
        3 => {
            // Hard Fault
            panic!("Hard Fault exception triggered!\n");
        }
        11 => {
            // SVCall
            todo!();
        }
        14 => {
            // PendSV
            todo!();
        }
        15 => {
            // SysTick
            todo!();
        }
        _ => {
            panic!("Unhandled exception number: {}.\n", exception_number);
        }
    }

    // Get back to next instruction and restore the registers pushed onto the stack
    unsafe {
        asm!(
            "mov pc, {}",
            "POP {{r0-r7}}",
            in(reg) return_pc
        );
    };
}

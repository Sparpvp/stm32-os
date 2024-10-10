use core::{arch::asm, ptr::read_volatile};

const ICSR_ADDR: u32 = 0xE000ED04;

#[no_mangle]
extern "C" fn rust_trap_handler() {
    // Save the callee-saved registers onto the stack
    // According to the arm calling convention, those are r4-r7
    unsafe {
        asm!("PUSH {{r4-r7}}");
    };

    let mut return_pc: u16;
    unsafe {
        // sp+24 should point to the PC
        asm!(
            "
            add sp, 24    
            mov r4, sp
            sub sp, 24
        "
        );
        asm!("LDR {}, [r4]", out(reg) return_pc);
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
            // panic!("Hard Fault exception triggered!\n");
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
            "POP {{r4-r7}}",
            "bx {}",
            in(reg) return_pc
        );
    };
}

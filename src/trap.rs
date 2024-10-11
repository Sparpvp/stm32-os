use core::{arch::asm, ptr::read_volatile};

const ICSR_ADDR: u32 = 0xE000ED04;

#[no_mangle]
extern "C" fn rust_trap_handler(stack_ptr: *const u32) {
    /*
        Current bugs:
            - stack_ptr gets overwritten by PUSH as soon as I do it
            - for some reason if I call this with blx it doesn't
                get processed at all
                => How do I return from here?
    */

    // Save the callee-saved registers onto the stack
    // According to the arm calling convention, those are r4-r7
    unsafe {
        asm!("PUSH {{r4-r7}}");
    };

    let mut return_pc: u32;
    unsafe {
        let pc_ptr = stack_ptr.byte_add(0x18 as usize);
        asm!("LDR {0}, [{1}]", out(reg) return_pc, in(reg) pc_ptr);
    }

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
            return_pc += 4;
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

    // Restore the registers pushed onto the stack
    unsafe {
        asm!(
            "
            mov r2, {0}
            POP {{r4-r7}}
        ", 
            in(reg) return_pc
        );
    };
}

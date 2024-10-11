use core::{arch::asm, ptr::read_volatile};

const ICSR_ADDR: u32 = 0xE000ED04;

#[no_mangle]
extern "C" fn rust_trap_handler(mut stack_ptr: *const u32) {
    /*
        Current bugs:
            -* stack_ptr gets overwritten by PUSH as soon as I do it
            - for some reason if I call this with blx it doesn't
                get processed at all
                => How do I return properly from here?? (in a way such that the cpu resets the
                interrupt state like it would by normally exiting the function)
    */

    // Save the callee-saved registers onto the stack
    // According to the arm calling convention, those are r4-r7

    // BIG TODO: QUESTO È UN ACCROCCHIO!!!
    /* Ho impostato stack_ptr come argomento mut per poterlo modificare.
        Siccome stack_ptr apparentemente è sullo stack, se modifico lo stack ptr con push/pop,
        verrebbe modificato l'indirizzo relativo a stack_ptr. (ancora non mi è chiaro perchè
        ciò debba avvenire con queste modalità).
        Per ovviare, salvo stack_ptr su r3, uno scratch register, pusho i registri callee-saved,
        e ripristino di nuovo stack_ptr da r3.
    */
    unsafe {
        asm!("
            mov r3, {0}
            PUSH {{r4-r7}}
            mov {1}, r3
        ", in(reg) stack_ptr, out(reg) stack_ptr
        );
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

use core::{arch::asm, ptr::read_volatile};

const ICSR_ADDR: u32 = 0xE000ED04;

// TODO: also rewrite this using only pure assembly
#[inline(always)]
unsafe fn get_pc(stack_ptr: *const u32) -> u32 {
    let mut base_pc: u32;

    unsafe {
        asm!(
            "
            MOV r3, {0}
            ADDS r3, #24
            LDR r2, [r3]
            MOV {1}, r2
        ", in(reg) stack_ptr, out(reg) base_pc
        );
    };

    base_pc
}

extern "C" {
    fn _update_pc(pc: u32, stack_ptr: *const u32);
    fn _get_pc(stack_ptr: *const u32) -> u32;
}

#[no_mangle]
extern "C" fn rust_trap_handler(mut stack_ptr: *const u32) {
    // TODO:
    /*
        Nonostante le ultime considerazioni, penso di dover salvare LR comunque.
        Se non posso popparlo dallo stack, verificare se posso metterlo in 
            un registro tra r2-r3, che non usano le mie funzioni in assembly.
            Non so se questa funzione in rust si può permettere di modificare da r0-r3.
            
        1) Save LR on stack
        2) POP LR from the stack

        La procedura di ritorno in ARMv6-M è diversa da quasi tutte le altre architetture.
        Pertanto, fare riferimento a QUESTE procedure:
            -> https://developer.arm.com/documentation/dui0203/j/handling-processor-exceptions/armv6-m-and-armv7-m-profiles/handling-an-exception

        e NON queste procedure, che sono per le altre architetture, ossia quelle che pensavo
            fossero quelle giuste finora.
            quindi, NON: https://developer.arm.com/documentation/dui0203/j/handling-processor-exceptions/armv6-and-earlier--armv7-a-and-armv7-r-profiles/handling-an-exception
            e NON: https://developer.arm.com/documentation/dui0203/h/handling-processor-exceptions/interrupt-handlers/simple-interrupt-handlers-in-c?lang=en
    */

    // Save the callee-saved registers onto the stack
    // According to the arm calling convention, those are r4-r7
    unsafe {
        asm!("
            MOV r3, {0}
            PUSH {{r4-r7, lr}}
            MOV {1}, r3
        ", in(reg) stack_ptr, out(reg) stack_ptr
        );
    };

    let mut return_pc = unsafe { get_pc(stack_ptr) };

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

    unsafe {
        // Inline assembly did not produce the expected behavior
        // Moved the func on trap.rs and marked as extern
        _update_pc(return_pc, stack_ptr);
    };

    // Restore the registers pushed onto the stack
    unsafe {
        asm!(
            "
            POP {{r4-r7, lr}}
            SUBS pc, lr, #4
        "
        );
    };
}

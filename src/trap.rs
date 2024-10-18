pub mod usart;

use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};

use usart::usart2_irq_receive;

const ICSR_ADDR: u32 = 0xE000ED04;
const NVIC_ICER: u32 = 0xE000E180;

#[inline(always)]
unsafe fn get_stack_alignment() -> usize {
    let curr_stack_ptr: usize;

    asm!("
        MOV {0}, sp
    ", out(reg) curr_stack_ptr
    );

    let remainder = curr_stack_ptr as usize % 8;
    remainder
}

extern "C" {
    fn _setup_frame(stack_ptr: *const u32) -> *const u32; // REQUIRES: r3 = lr
    fn _update_pc(pc: u32, stack_ptr: *const u32);
    fn _get_pc(stack_ptr: *const u32) -> u32;
}

#[no_mangle]
extern "C" fn rust_trap_handler(mut stack_ptr: *const u32) {
    /*
        La procedura di ritorno in ARMv6-M è diversa da quasi tutte le altre architetture.
        Pertanto, fare riferimento a QUESTE procedure:
            -> https://developer.arm.com/documentation/dui0203/j/handling-processor-exceptions/armv6-m-and-armv7-m-profiles/handling-an-exception

        e NON queste procedure, che sono per le altre architetture, ossia quelle che pensavo
            fossero quelle giuste finora.
            quindi, NON: https://developer.arm.com/documentation/dui0203/j/handling-processor-exceptions/armv6-and-earlier--armv7-a-and-armv7-r-profiles/handling-an-exception
            e NON: https://developer.arm.com/documentation/dui0203/h/handling-processor-exceptions/interrupt-handlers/simple-interrupt-handlers-in-c?lang=en
    */
    assert_eq!(unsafe { get_stack_alignment() == 0 }, true);

    // Save the callee-saved registers onto the stack
    // According to the arm calling convention, those are r4-r7
    stack_ptr = unsafe {
        asm!("MOV r3, lr", clobber_abi("aapcs"));
        // Modifying the current sp, for some reason, modifies the stack_ptr variable too.
        // So I return the stack_ptr that was passed as an argument.
        _setup_frame(stack_ptr)
    };

    // If we want to return at a different location on main,
    // We just need to modify the return program counter from here.
    let mut return_pc = unsafe { _get_pc(stack_ptr) };

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
            // return_pc += 4; // Here for testing purposes
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
        44 => {
            // USART2 - The formula is IRQ(n-1) = n+15
            // -> Hence IRQ28 is USART2, so IRQ(28) = IRQ(29-1) = 29+15 = 44
            usart2_irq_receive();
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

    // Clear interrupt from NVIC
    let nvic_icer = NVIC_ICER as *mut u32;
    unsafe {
        // On writes: 1 - Disables the associated interrupt
        // https://developer.arm.com/documentation/ddi0419/c/System-Level-Architecture/System-Address-Map/Nested-Vectored-Interrupt-Controller--NVIC/Interrupt-Clear-Enable-Register--NVIC-ICER?lang=en
        write_volatile(nvic_icer, 1);
    };

    // Unmask Interrupts and Restore the registers pushed onto the stack
    unsafe {
        asm!(
            "
            CPSIE i
            POP {{r3, r4-r7}}
            MOV lr, r3
        "
        );
    };
}

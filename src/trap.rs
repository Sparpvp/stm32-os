pub mod critical_section;
pub mod irq_handlers;

use core::{
    arch::asm,
    ptr::{read_volatile, write_volatile},
};

use irq_handlers::usart2_irq_receive;
use volatile_register::RW;

const ICSR_ADDR: u32 = 0xE000ED04;
const NVIC_ICER: u32 = 0xE000E180;

#[no_mangle]
#[used]
pub static mut FIRST_CTX_SWITCH: bool = true; // Resetted in ASM

extern "C" {
    fn _setup_frame(stack_ptr: *const u32) -> *const u32; // REQUIRES: r3 = lr
    fn _update_pc(pc: u32, stack_ptr: *const u32);
    fn _get_pc(stack_ptr: *const u32) -> u32;
}

fn pend_sv_set() {
    // Bit 28: PENDSVSET
    let icsr = unsafe { &mut *(ICSR_ADDR as *mut RW<u32>) };
    unsafe {
        icsr.modify(|m| m | (1 << 28));
    };
}

#[no_mangle]
extern "C" fn rust_trap_handler(stack_ptr: *const u32) {
    /*
        The return procedure for ARMv6-M is different from almost all the other architectures.
        Hence, I have to refer to THESE docs:
            -> https://developer.arm.com/documentation/dui0203/j/handling-processor-exceptions/armv6-m-and-armv7-m-profiles/handling-an-exception
        and NOT these, which are for the other architectures
            so, NOT: https://developer.arm.com/documentation/dui0203/j/handling-processor-exceptions/armv6-and-earlier--armv7-a-and-armv7-r-profiles/handling-an-exception
            and NOT: https://developer.arm.com/documentation/dui0203/h/handling-processor-exceptions/interrupt-handlers/simple-interrupt-handlers-in-c?lang=en
    */

    // Save LR at the beginning to return correctly
    let original_lr: u32;
    unsafe {
        asm!("MOV {}, LR", out(reg) original_lr);
    }

    assert_eq!(stack_ptr as usize % 8 == 0, true);

    // If we want to return at a different location on main,
    //  we just need to modify the return program counter in this variable.
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
        }
        11 => {
            // SVCall
            todo!();
        }
        15 => {
            // SysTick
            // Do context switch
            pend_sv_set();
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
            MOV LR, {0}
        ",
            in(reg) original_lr,
        );
    };
}

/* --------------------------- */
/*     Interrupt Handler       */

// The Cortex-M0 doc says: 
// 0x0 Initial SP value -> Reset -> NMI -> HardFault -> Reserved -> ...
// https://developer.arm.com/documentation/dui0497/a/the-cortex-m0-processor/exception-model/vector-table

.section .isr_vector, "a"
.global _isr_vector
.thumb
.thumb_func

_isr_vector:
    .word _stack_end
    .word _start
    .word rust_trap_handler
    .word rust_trap_handler
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word rust_trap_handler
    .word 0
    .word 0
    .word rust_trap_handler
    .word rust_trap_handler
    /* 
        Here we're setting rust_trap_handler for each vector entry.
        It'll be called in different circumstances and it's up to the handler to distinguish each case.
    */
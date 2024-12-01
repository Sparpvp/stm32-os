/* --------------------------- */
/*     Interrupt Handler       */

// The Cortex-M0 doc says: 
// 0x0 Initial SP value -> Reset -> NMI -> HardFault -> Reserved -> ...
// https://developer.arm.com/documentation/dui0497/a/the-cortex-m0-processor/exception-model/vector-table

.section ".text.trapwrapper"
.global _context_switch
.global _trap_wrapper
.thumb
.thumb_func

_trap_wrapper:
    CPSID i     // mask everything! We don't want to be interrupted.
    push {r0, r1, r2, r3, lr}
    mrs r0, msp
    mov r1, lr
    mov r2, #0b100
    tst r1, r2  // bit 3 of LR tells us whether we'll be using msp or psp
    bne psp_jmp
    b call_handler
psp_jmp:
    mrs r0, psp
call_handler:
    // r0-r3 registers are saved by the cpu on exception entry
    ldr r1, =rust_trap_handler
    BLX r1       // r0 is used as the first parameter to be passed to the function
    CPSIE i
    pop {r0, r1, r2, r3, pc}

/* --------------------------- */
/*        Helper Funcs         */
.global _update_pc
.thumb
.thumb_func
_update_pc:
    // r0 -> pc
    // r1 -> stack_ptr
    ADD r1, #24
    STR r0, [r1]
    BX lr

.global _get_pc
.thumb
.thumb_func
_get_pc:
    // r0 -> stack_ptr; ret value
    ADD r0, #24
    LDR r1, [r0]
    MOV r0, r1
    BX lr

.global _extract_svc_comment
.thumb
.thumb_func
_extract_svc_comment:
    /* SUB r0, r0, #4
    LDR r0, [r0]
    MOV r1, #0xFF
    LSL r1, r1, #24
    MVN r1, r1
    AND r0, r0, r1
    BX lr */

    // r0 -> stack pointer; ret value
    LDR r1, [r0];
    MOV r0, r1
    BX lr

/* ----------------------------- */
/*         Vector Table          */

.section .isr_vector, "a"
.global _isr_vector
.thumb
.thumb_func

_isr_vector:
    .word _stack_end
    .word _start
    .word _trap_wrapper     // NMI
    .word _trap_wrapper     // Hard Fault
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word _trap_wrapper     // SVCall
    .word 0
    .word 0
    .word _context_switch   // PendSV
    .word _trap_wrapper     // SysTick
    .word 0                 // IRQ0. All IRQs < USART2 are currently unhandled.
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word 0
    .word _trap_wrapper     // IRQ 28 - USART2
    /* 
        Here we're setting a procedure that calls the trap handler 
            giving the correct stack pointer for each vector entry.
        It'll be called in different circumstances 
            and it's up to the handler to distinguish each case.
    */
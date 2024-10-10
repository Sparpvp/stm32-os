.section ".text.boot"
.global _start
.global _isr_vector
.thumb
.thumb_func

_start:
    /* on Cortex-M0 we don't need to halt 
        other threads since there aren't any */

    /* clear the bss as the C runtime requires it
        (bss is aligned to 4 bytes) */
    ldr r0, =_bss_start
    ldr r1, =_bss_end
setzero: 
    mov r2, #0
    str r2, [r0]
    add r0, #4
    cmp r0, r1
    blo setzero // branch if r0 is less than r1
    
    // set stack pointer to the top, right after the kernel stack (1K-long)
    // (remember that the stack is descending)
    ldr r2, =_stack_end
    mov sp, r2

    // set the reference to kmain
    ldr r3, =kmain
    // jump to rust main!
    blx r3

halt:
    wfe
    b halt
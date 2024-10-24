.section ".text.boot"
.global _start
.global _isr_vector
.thumb
.thumb_func

_start:
    /* on Cortex-M0 we don't need to halt 
        other threads since there aren't any */

    /* copy data segment from flash to ram */
    ldr r0, =_flash_data_start
    ldr r1, =_data_start
    ldr r3, =_data_end
copy_data:
    ldrb r2, [r0]
    strb r2, [r1]
    add r0, r0, #1
    add r1, r1, #1
    cmp r1, r3
    blo copy_data // loop if there's more data

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
    
    // set stack pointer to the top, right after the kernel stack (2K-long)
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
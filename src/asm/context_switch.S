/* -------------------------------------- */
/*     Context Switch (PendSV Handler)    */

.section ".text.ctxswitch"
.global _context_switch
.thumb
.thumb_func

_context_switch:
    CPSID i

    PUSH {r0}
    LDR r0, =CURR_PROC
    ADD r0, r0, #4

    // Save r1-r6 into the current process
    // We still have to save r0
    // We can't use the post-indexed syntax as it isn't supported on Thumb.
    STR r1, [r0]
    ADD r0, r0, #4
    STR r2, [r0]
    ADD r0, r0, #4
    STR r3, [r0]
    ADD r0, r0, #4
    STR r4, [r0]
    ADD r0, r0, #4
    STR r5, [r0]
    ADD r0, r0, #4
    STR r6, [r0]
    ADD r0, r0, #4

    // Check whether we're using MSP or PSP and save it
    MOV r1, lr
    MOV r2, #0b100
    TST r1, r2
    BNE str_psp
    B str_msp
str_psp:
    MRS r1, psp
    STR r1, [r0]
    ADD r0, r0, #4
str_msp:
    MRS r1, msp
    STR r1, [r0]
    ADD r0, r0, #4

    // Load the link register, get it from the interrupt-generated stack frame
    SUB sp, sp, #0x18 // (= - 0x4(PUSH) - 0x14(LR))
    LDR r1, [sp]
    STR r1, [r0]
    ADD r0, r0, #4 
    // Load the PC with the same method
    SUB sp, sp, #4
    LDR r1, [sp]
    STR r1, [r0]
    ADD r0, r0, #4
    ADD sp, sp, #0x1C // (= 0x18 + 0x4)

    // Load the flags
    MRS r1, APSR
    STR r1, [r0]
    ADD r0, r0, #4
    MRS r1, PRIMASK
    STR r1, [r0]
    ADD r0, r0, #4
    MRS r1, CONTROL
    STR r1, [r0]
    ADD r0, r0, #4

    // Set process to ready status
    // Skip stack_base variable (*mut T -> 4 byte size)
    ADD r0, r0, #4
    MOV r1, #1
    STR r1, [r0]

    // Save also r0, which contained the CURR_PROC addy until now
    LDR r1, =CURR_PROC
    POP {r0}
    STR r0, [r1]

    // TODO: Restore context

    CPSIE i
    BX lr
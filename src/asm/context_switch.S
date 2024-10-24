/* -------------------------------------- */
/*     Context Switch (PendSV Handler)    */

.section ".text.ctxswitch"
.global _context_switch
.thumb
.thumb_func

_context_switch:
    CPSID i

    PUSH {r0}
    LDR r0, =FIRST_CTX_SWITCH
    LDRB r0, [r0]
    CMP r0, #1
    BEQ restore_context

    LDR r0, =CURR_PROC
    ADD r0, r0, #4

    // Save r1-r7 into the current process
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
    STR r7, [r0]
    ADD r0, r0, #4

    // Check whether we were using MSP or PSP and save it
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
    // Note: stacking on PSP (on armv6) happens only if the mode at
    //  exception entry was mode_thread. So we'll always be using MSP
    // Note: here we're assuming sp aliases for msp. If not, we'll save msp in a reg.
    ADD sp, sp, #24 // (= 4(PUSH) + 0x14(LR))
    LDR r1, [sp]
    STR r1, [r0]
    ADD r0, r0, #4 
    // Load the PC with the same method
    ADD sp, sp, #4
    LDR r1, [sp]
    STR r1, [r0]
    ADD r0, r0, #4
    SUB sp, sp, #0x1C // (= 0x18 + 0x4)

    // Load the flags
    MRS r1, APSR
    STR r1, [r0]
    ADD r0, r0, #4
    // PRIMASK has been overwritten by CPSID i. 
    // We set that flag manually to 0.
    MOV r1, #0
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

    // Now we'll load the next process' context
    
restore_context:
    // Reset first context switch flag
    LDR r0, =FIRST_CTX_SWITCH
    MOV r1, #0
    STRB r1, [r0]

    // Invoke the scheduler and put the next process into CURR_PROC
    PUSH {lr}       // Save LR in r3 to return
    LDR r0, =next_proc
    BLX r0

    // Get the new to-be-loaded process address
    LDR r0, =CURR_PROC

    // Modify the interruption stack frame with the new process context
    ADD sp, sp, #4  // Adjust stack pointer to the stacking frame
    LDR r1, [r0]    // Get then store r0
    STR r1, [sp]
    ADD sp, sp, #4
    ADD r0, r0, #4
    LDR r1, [r0]    // Get then store r1
    STR r1, [sp]
    ADD sp, sp, #4
    ADD r0, r0, #4
    LDR r1, [r0]    // Get then store r2
    STR r1, [sp]
    ADD sp, sp, #4
    ADD r0, r0, #4
    LDR r1, [r0]    // Get then store r3
    STR r1, [sp]
    ADD sp, sp, #8
    ADD r0, r0, #24
    LDR r1, [r0]    // Get then store LR
    STR r1, [sp]
    ADD sp, sp, #4
    ADD r0, r0, #4
    LDR r1, [r0]    // Get then store PC
    STR r1, [sp]
    ADD sp, sp, #4
    ADD r0, r0, #4
    LDR r1, [r0]    // Get then store PSR
    STR r1, [sp]
    SUB sp, sp, #4  // Reset the previous 4 bytes added

    // From now r0-r3 are scratchable
    // Reset stack pointer to the base during the interrupt
    SUB sp, sp, #0x1C
    // Reset r0 after r3 addy
    // I'm too lazy to calculate the offset, we'll just re-load it
    LDR r0, =CURR_PROC
    ADD r0, r0, #16

    // We can't modify r4-r7 from now
    LDR r4, [r0]
    ADD r0, r0, #4
    LDR r5, [r0]
    ADD r0, r0, #4
    LDR r6, [r0]
    ADD r0, r0, #4
    LDR r7, [r0]
    ADD r0, r0, #4

    // Modify the current stack pointer
    // Since we have only one kernel process (that uses msp)
    //  we can avoid moving the stack pointer if we'll be using msp.
    ADD r0, r0, #20
    // If [r0] & 0b10 is 1, we'll be using the psp
    LDR r1, [r0]
    SUB r0, r0, #20
    MSR CONTROL, r1
    MOV r2, #0b10
    TST r1, r2
    BNE return_psp
    B return
return_psp:
    LDR r1, [r0]    // r0 points to curr_sp
    MSR psp, r1
return:
    ADD r0, r0, #12
    LDR r1, [r0]
    MSR APSR, r1

    // Restore the previously-saved link register
    POP {r3}
    MOV lr, r3

    CPSIE i
    BX lr

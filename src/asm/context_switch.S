/* -------------------------------------- */
/*     Context Switch (PendSV Handler)    */

.section ".text.ctxswitch"
.global _context_switch
.thumb
.thumb_func

_context_switch:
    CPSID i

    // Since PendSV is low priority, it cannot preempt any handler.
    // Therefore we don't need to worry about
    //  context-switching from a handler preemption.
    PUSH {r0}
    // Check if we're doing the first context switch. In that
    //  case, just jump to the first available process.
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

    // We can 'freely' use the registers now as they've been saved.
    // Save the stack pointer into the process/thread control block.
    //  We're sure we were using PSP here.
    MRS r2, psp
    STR r2, [r0]
    ADD r0, r0, #4

    // Load the link register, get it from the interrupt-generated stack frame
    // Note: stacking on PSP (on armv6) happens only if the mode at
    //  exception entry was mode_thread. Again, by now it's certain it has used psp.
    ADD r2, r2, #0x14 // (= 0x14(LR)). R2 contains PSP
    LDR r1, [r2]
    STR r1, [r0]
    ADD r0, r0, #4 
    // Load the PC with the same method
    ADD r2, r2, #4
    LDR r1, [r2]
    STR r1, [r0]
    ADD r0, r0, #4

    // Load the flags
    // Get PSR from interruption stack frame
    ADD r2, r2, #4
    LDR r1, [r2] // Get xPSR
    STR r1, [r0] // Store xPSR
    ADD r0, r0, #4
    // If we got here PRIMASK must have been zero. 
    //  hence it's superflous to overwrite with 0
    // Same for CONTROL which is always 2
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
    // Invoke the scheduler and put the next process into CURR_PROC
    PUSH {lr}       // Save LR in r3 to return
    LDR r0, =next_proc
    BLX r0

    // Reset first context switch flag
    LDR r0, =FIRST_CTX_SWITCH
    MOV r1, #0
    STRB r1, [r0]

    // Get the new to-be-loaded process address, to retrieve the PSP
    LDR r0, =CURR_PROC

    // Modify the interruption stack frame (on PSP) with the new process context
    // Adjust stack pointer to the next process frame
    MOV r1, r0
    ADD r1, r1, #32
    LDR r2, [r1]
    MSR psp, r2

    // r2 contains the PSP address where the interruption stack frame starts
    // r0 contains the current process address

    LDR r1, [r0]    // Get then store r0
    STR r1, [r2]
    ADD r2, r2, #4
    ADD r0, r0, #4
    LDR r1, [r0]    // Get then store r1
    STR r1, [r2]
    ADD r2, r2, #4
    ADD r0, r0, #4
    LDR r1, [r0]    // Get then store r2
    STR r1, [r2]
    ADD r2, r2, #4
    ADD r0, r0, #4
    LDR r1, [r0]    // Get then store r3
    STR r1, [r2]
    ADD r2, r2, #8
    ADD r0, r0, #24
    LDR r1, [r0]    // Get then store LR
    STR r1, [r2]
    ADD r2, r2, #4
    ADD r0, r0, #4
    LDR r1, [r0]    // Get then store PC
    STR r1, [r2]
    ADD r2, r2, #4
    ADD r0, r0, #4
    LDR r1, [r0]    // Get then store PSR
    STR r1, [r2]
    SUB r2, r2, #4  // Reset the previous 4 bytes added

    // From now r0-r3 are scratchable
    // Reset stack pointer to the base during the interrupt
    SUB r2, r2, #0x1C
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

    // ADD r0, r0, #12
    // LDR r1, [r0]
    // MSR APSR, r1

    // Restore the previously-saved link register
    POP {r3}
    MOV lr, r3

    CPSIE i
end_ret:
    BX lr

.global _switch_to_psp
.thumb
.thumb_func

_switch_to_psp:
    MSR psp, r0
    MOV r1, #2
    MSR CONTROL, r1
    ISB
    BX lr
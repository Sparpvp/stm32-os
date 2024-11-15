#![no_std]
#![no_main]

extern crate alloc;
use core::arch::asm;

pub mod allocator;
pub mod circ_buffer;
pub mod panic;
pub mod peripherals;
pub mod process;
pub mod scheduler;
pub mod shell;
pub mod tasks;
pub mod trap;

use allocator::memory::FreeList;
use circ_buffer::CircularBuffer;
use peripherals::{
    core::{IPR, IT_PENDSV},
    rcc::{Rcc, RccConfig},
    usart::UsartConfig,
    Config, Peripherals,
};
use process::Process;
use shell::shell;
use tasks::*;

#[no_mangle]
extern "C" fn kmain() -> ! {
    // hprintln!("Xemo vivi!").unwrap();

    /* Current TODO:
        - Create a "Kernel Process" that will be automatically scheduled that acts as a Shell.
            In this way I can avoid the user processes taking all the CPU time and not allowing
            ME to interact with the OS. For example, I could create a kernel process that records
                the input received from USART via interrupts, and then implement a functionality
                    that removes a process from the scheduler list, if the user asked to do so.
        => This comes with implementing some things
        TICK 1. USART Interrupts
        TICK 2. Circular Buffer for the stdin
            Note that pending interrupts while PRIMASK is set to 1 will be executed right after
                PRIMASK is resetted to 0. Hence the USART data isn't lost.

        TICK - Implement SysTick interrupt to do... context switches! using a Round-Robin algorithm.
            I just keep an internal static that is "mod-ed" modulo PROC_NUM when I get to the end
                of the list. In simpler words, once I get to the end of the list, I reset the
                    counter to 0, such that the scheduler will grep all the procs from the beginning.
        pending - Context Switches indeed
            => This is going to be a pain
            - Ensure that PendSV handler is being called
            - Ensure that PendSV is able to return correctly (disabling interrupt state, popping the interrupt stack frame)
            - Do I have to write the entire context switch in assembly?
                - That's crazy! How do I handle MaybeUninit<Stuff>.stuff.stuff[index] type of thing?

        - print! macro writing on USART

        So cool!
    */

    let rcc = Rcc::new(RccConfig {
        sysclk: 8_000_000,
        pclk: 8_000_000,
    });
    let config = Config {
        usart_config: UsartConfig { baud_rate: 9600 },
    };

    FreeList::init();
    CircularBuffer::init();
    IPR::set_priority(IT_PENDSV, 43); // We don't want USART to preempt PendSV
    let _p = Peripherals::init(rcc, config);

    // Spawn function takes care of all the final initialization.
    // It includes SysTick interrupts and Scheduler init (psp switch).
    Process::spawner().new(shell).new(beef).spawn();

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

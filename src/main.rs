#![no_std]
#![no_main]

extern crate alloc;
use core::arch::asm;

use alloc::vec::Vec;

pub mod allocator;
pub mod circ_buffer;
pub mod panic;
pub mod peripherals;
pub mod process;
pub mod scheduler;
pub mod shell;
pub mod trap;

use allocator::memory::{free, zalloc_block, FreeList};
use circ_buffer::CircularBuffer;
use cortex_m_semihosting::hprintln;
use peripherals::{
    core::{SysTick, IPR, IT_PENDSV},
    rcc::{Rcc, RccConfig},
    usart::{UsartConfig, G_USART},
    Config, Peripherals,
};
use process::Process;
use scheduler::Scheduler;
use shell::shell;

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

        pending - Implement SysTick interrupt to do... context switches! using a Round-Robin algorithm.
            I just keep an internal static that is "mod-ed" modulo PROC_NUM when I get to the end
                of the list. In simpler words, once I get to the end of the list, I reset the
                    counter to 0, such that the scheduler will grep all the procs from the beginning.
        - Context Switches indeed
        - print! macro writing on USART

        So cool!
    */

    // let heap1 = zalloc_block(50);
    // free(heap1);
    // let mut vec2 = Vec::from([1, 24, 235]);
    // vec2.push(132);
    // let a = vec2[1];
    // let ptr1 = zalloc_block(12);
    // let ptr2 = zalloc_block(20);
    // unsafe {
    //     let unaligned_ptr = (0x080003 as *mut u8);
    //     *unaligned_ptr = 5; // Hard Fault
    // }
    // p.usart.write('a' as u8, &p.rcc);
    // p.usart.read(&p.rcc);

    FreeList::init();
    Scheduler::init();
    CircularBuffer::init();
    Process::new_kernel_proc(shell).enqueue();
    IPR::set_priority(IT_PENDSV, 255);

    let rcc = Rcc::new(RccConfig {
        sysclk: 8_000_000,
        pclk: 8_000_000,
    });
    let config = Config {
        usart_config: UsartConfig { baud_rate: 9600 },
    };
    let p = Peripherals::init(rcc, config);

    SysTick::enable();

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

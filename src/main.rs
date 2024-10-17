#![no_std]
#![no_main]

extern crate alloc;
use alloc::vec::Vec;

pub mod allocator;
pub mod panic;
pub mod peripherals;
pub mod process;
pub mod scheduler;
pub mod trap;

use allocator::memory::{free, zalloc_block, FreeList};
use cortex_m_semihosting::hprintln;
use peripherals::{
    rcc::{Rcc, RccConfig},
    usart::UsartConfig,
    Config, Peripherals,
};
use scheduler::ProcListWrapper;

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
        1. USART Interrupts
        2. Circular Buffer for the stdin
        Note that pending interrupts while PRIMASK is set to 1 will be executed right after
            PRIMASK is resetted to 0. Hence the USART data isn't lost.

        - Implement SysTick interrupt to do... context switches! using a Round-Robin algorithm.
            I just keep an internal static that is "mod-ed" modulo PROC_NUM when I get to the end
                of the list. In simpler words, once I get to the end of the list, I reset the
                    counter to 0, such that the scheduler will grep all the procs from the beginning.
        - Context Switches indeed

        So cool!
    */

    FreeList::init();
    ProcListWrapper::init();

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

    let rcc = Rcc::new(RccConfig {
        sysclk: 8_000_000,
        pclk: 8_000_000,
    });
    let config = Config {
        usart_config: UsartConfig { baud_rate: 9600 },
    };
    let p = Peripherals::take(rcc, config);

    // p.usart.write('a' as u8, &p.rcc);
    // p.usart.read(&p.rcc);

    loop {}
}

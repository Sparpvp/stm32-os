#![no_std]
#![no_main]

extern crate alloc;
use core::arch::asm;
#[macro_use]
pub mod panic;

pub mod allocator;
pub mod circ_buffer;
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
    Process::spawner()
        .new(shell)
        .new_with_stack(beef, 256)
        .spawn();

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

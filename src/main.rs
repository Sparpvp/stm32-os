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
pub mod syscall;
pub mod tasks;
pub mod trap;

use allocator::memory::FreeList;
use circ_buffer::CircularBuffer;
use dispatcher::ProcessIdentifier;
use peripherals::{
    core::{IPR, IT_PENDSV},
    rcc::{ClockSource, PPREScaler, Rcc, RccConfig, SysClkMultiplier},
    Config, Peripherals, UsartConfig,
};
use process::Process;
use shell::shell;
use tasks::*;

#[no_mangle]
extern "C" fn kmain() -> ! {
    let config = Config {
        rcc_config: RccConfig {
            source: ClockSource::PLL,
            sysclk: SysClkMultiplier::PLL_MUL2,
            pclk: PPREScaler::AS_SYSCLK,
        },
        usart_config: UsartConfig { baud_rate: 9600 },
    };
    let rcc = Rcc::new(&config.rcc_config);

    FreeList::init();
    CircularBuffer::init();
    IPR::set_priority(IT_PENDSV, 43); // We don't want USART to preempt PendSV
    let _p = Peripherals::init(rcc, config);

    // We give each function an alias/name that can be used to retrieve the fn address at runtime.
    // This must be done before spawning the processes, as it is required as a proof of construction for the spawn.
    let s = ProcessIdentifier::saver()
        .add("beef", beef)
        .add("sbeaf", sbeaf);

    // Spawn function takes care of all the final initialization.
    // It includes SysTick interrupts and Scheduler init (psp switch).
    Process::spawner()
        .new_with_stack(shell, 4096)
        .new(beef)
        .spawn(s);

    loop {
        unsafe {
            asm!("wfi");
        }
    }
}

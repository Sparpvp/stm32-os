#![no_std]
#![no_main]

pub mod allocator;
pub mod panic;
pub mod peripherals;
mod trap;

use allocator::{zalloc_block, FreeList};
// use cortex_m_semihosting::hprintln;
use peripherals::{
    rcc::{Rcc, RccConfig},
    usart::UsartConfig,
    Config, Peripherals,
};

#[no_mangle]
extern "C" fn kmain() -> ! {
    // hprintln!("Xemo vivi!").unwrap();

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

    // FreeList::init();
    // let ptr1 = unsafe { zalloc_block(12) };
    // let ptr2 = unsafe { zalloc_block(20) };

    // p.usart.write('a' as u8, &p.rcc);
    // p.usart.read(&p.rcc);

    loop {}
}

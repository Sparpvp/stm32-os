#![no_std]
#![no_main]

pub mod allocator;
pub mod panic;
pub mod peripherals;

use cortex_m_semihosting::hprintln;
use peripherals::{
    rcc::{Rcc, RccConfig},
    usart::UsartConfig,
    Config, Peripherals,
};

#[no_mangle]
extern "C" fn kmain() -> ! {
    // hprintln!("Xemo vivi!").unwrap();

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

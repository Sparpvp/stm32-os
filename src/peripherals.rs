use gpio::GPIOA;
use rcc::{Rcc, RCC};
use usart::{Usart, UsartConfig};

pub mod gpio;
pub mod rcc;
pub mod usart;

pub struct Peripherals<'a> {
    pub rcc: RCC,
    pub gpioa: GPIOA<'a>,
    pub usart: Usart<'a>,
}

pub struct Config {
    // gpioa_config: GPIOAConfig,
    pub usart_config: UsartConfig,
}

impl<'a> Peripherals<'a> {
    pub fn take(rcc: Rcc, c: Config) -> Peripherals<'a> {
        let rcc_freeze = rcc.freeze();
        let gpioa = GPIOA::new();
        let usart = Usart::new(c.usart_config);

        Peripherals {
            rcc: rcc_freeze,
            gpioa: gpioa,
            usart: usart,
        }
    }
}

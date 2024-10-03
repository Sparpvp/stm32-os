use gpio::GPIOA;
use rcc::Rcc;
use usart::{Usart, UsartConfig};

pub mod gpio;
pub mod rcc;
pub mod usart;

// TODO implement singleton
pub struct Peripherals<'a> {
    pub rcc: Rcc,
    pub gpioa: GPIOA,
    pub usart: Usart<'a>,
}

impl<'a> Peripherals<'a> {
    pub fn take() -> Peripherals<'a> {
        let rcc = Rcc::new();
        let gpioa = GPIOA::new();
        let usart = Usart::new(UsartConfig { baud_rate: 9600 });

        Peripherals {
            rcc: rcc,
            gpioa: gpioa,
            usart: usart,
        }
    }
}

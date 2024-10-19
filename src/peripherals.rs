use exti::EXTI;
use gpio::GPIOA;
use rcc::{Rcc, RCC};
use usart::{Usart, UsartConfig, G_USART};

pub mod exti;
pub mod gpio;
pub mod rcc;
pub mod usart;
pub mod core;

pub struct Peripherals<'a> {
    pub rcc: RCC,
    pub gpioa: GPIOA<'a>,
    pub usart: Option<Usart<'a>>,
}

pub struct Config {
    // gpioa_config: GPIOAConfig,
    pub usart_config: UsartConfig,
}

impl<'a> Peripherals<'a> {
    pub fn init(rcc: Rcc, c: Config) -> Peripherals<'a> {
        assert_eq!(unsafe { G_USART.is_none() }, true);

        let rcc_freeze = rcc.freeze();
        let gpioa = GPIOA::new();
        let usart = Usart::new(c.usart_config);
        unsafe {
            G_USART.replace(usart);
        };
        EXTI::unmask_usart2();

        Peripherals {
            rcc: rcc_freeze,
            gpioa: gpioa,
            usart: None,
        }
    }
}

use exti::EXTI;
use gpio::GPIOA;
use rcc::{Rcc, RccConfig, RCC};
use usart::{Usart, G_USART};

pub mod core;
pub mod exti;
pub mod gpio;
pub mod rcc;
pub mod usart;

pub struct Peripherals<'a> {
    pub rcc: RCC,
    pub gpioa: GPIOA<'a>,
    pub usart: Option<Usart<'a>>,
}

pub struct Config {
    pub rcc_config: RccConfig,
    pub usart_config: UsartConfig,
}

pub struct UsartConfig {
    pub baud_rate: u32,
}

impl<'a> Peripherals<'a> {
    pub fn init(rcc: Rcc, c: Config) -> Peripherals<'a> {
        assert_eq!(unsafe { G_USART.is_none() }, true);

        let rcc_freeze = rcc.freeze();
        let gpioa = GPIOA::new();
        let usart = Usart::new(c);
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

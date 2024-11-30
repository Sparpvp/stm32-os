use alloc::borrow::ToOwned;
use core::fmt::Write;
use core::str;
use volatile_register::RW;

use super::rcc::{ClockSource, SysClkMultiplier, HSI_DEFAULT_CLK, RCC};
use super::Config;

const USART2_ADDR: usize = 0x4000_4400;

pub struct Usart<'a> {
    pub rb: &'a mut RegisterBlock,
}

#[repr(C)]
pub struct RegisterBlock {
    cr1: RW<u32>,
    cr2: RW<u32>,
    cr3: RW<u32>,
    brr: RW<u32>,
    gtpr: RW<u32>,
    rtor: RW<u32>,
    rqr: RW<u32>,
    isr: RW<u32>,
    pub icr: RW<u32>,
    pub rdr: RW<u32>,
    tdr: RW<u32>,
}

pub static mut G_USART: Option<Usart> = None;

impl<'a> Usart<'a> {
    pub(in crate::peripherals) fn new(c: Config) -> Usart<'a> {
        let g = unsafe { &mut *(USART2_ADDR as *mut RegisterBlock) };

        unsafe {
            if c.rcc_config.source == ClockSource::HSI {
                g.brr
                    .write(c.rcc_config.sysclk.0 / c.usart_config.baud_rate);
            } else if c.rcc_config.source == ClockSource::PLL {
                let ppre_divisor: u32 = match c.rcc_config.pclk.0 {
                    0 => 1,
                    4 => 2,
                    5 => 4,
                    6 => 8,
                    7 => 16,
                    _ => unreachable!(),
                };

                g.brr.write(
                    (HSI_DEFAULT_CLK / 2) * (c.rcc_config.sysclk.0 + 2)
                        / ppre_divisor
                        / c.usart_config.baud_rate,
                );
            }

            // RXNEIE (5): Interrupt on receive, TE (3): Transmitter Enable,
            // RE (2): Receiver Enable, UE (0): USART Enable.
            g.cr1.write((1 << 5) | (1 << 3) | (1 << 2) | (1 << 0));
            g.cr2.write(0);
            g.cr3.write(0);
        }

        Usart { rb: g }
    }

    pub fn write(&self, data: u8) {
        // Configure USART as transmitter
        unsafe {
            // Wait for TXE (clear to send)
            while (self.rb.isr.read() & (1 << 7)) == 0 {}
            // Send byte
            self.rb.tdr.write(data as u32 & 0xFF);
            // Wait for TC (Transmission Complete)
            while (self.rb.isr.read() & (1 << 6)) == 0 {}
            // Notify end
            self.rb.icr.modify(|m| m | (1 << 6));
        }
    }

    pub fn write_string(&self, s: &str) {
        unsafe {
            s.as_bytes().into_iter().for_each(|w| {
                while (self.rb.isr.read() & (1 << 7)) == 0 {}
                self.rb.tdr.write(*w as u32 & 0xFF);
                while (self.rb.isr.read() & (1 << 6)) == 0 {}
                self.rb.icr.modify(|m| m | (1 << 6));
            });
        }
    }

    pub fn read_polling(&self) {
        while (self.rb.isr.read() & (1 << 5)) == 0 {}
        let r = (self.rb.rdr.read() & 0xFF) as u8;
        // TODO use yet-to-define print macro and print r
    }
}

impl<'a> Write for Usart<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.write_string(s);

        Ok(())
    }
}

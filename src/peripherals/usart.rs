use core::str;

use volatile_register::RW;

use super::rcc::RCC;

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
    icr: RW<u32>,
    pub rdr: RW<u32>,
    tdr: RW<u32>,
}

pub static mut G_USART: Option<Usart> = None;

pub struct UsartConfig {
    pub baud_rate: u32,
}

impl<'a> Usart<'a> {
    pub(in crate::peripherals) fn new(c: UsartConfig) -> Usart<'a> {
        let g = unsafe { &mut *(USART2_ADDR as *mut RegisterBlock) };

        unsafe {
            // TODO! I've hardcoded 8mhz as RCC PCLK frequency
            g.brr.write(8_000_000 / c.baud_rate);
            // RXNEIE (5): Interrupt on receive, TE (3): Transmitter Enable,
            // RE (2): Receiver Enable, UE (0): USART Enable.
            g.cr1.write((1 << 5) | (1 << 3) | (1 << 2) | (1 << 0));
            g.cr2.write(0);
            g.cr3.write(0);
        }

        Usart { rb: g }
    }

    pub fn write(&self, data: u8, _rcc: &RCC) {
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

    pub fn write_string(&self, s: &str, _rcc: &RCC) {
        unsafe {
            s.as_bytes().into_iter().for_each(|w| {
                while (self.rb.isr.read() & (1 << 7)) == 0 {}
                self.rb.tdr.write(*w as u32 & 0xFF);
                while (self.rb.isr.read() & (1 << 6)) == 0 {}
                self.rb.icr.modify(|m| m | (1 << 6));
            });
        }
    }

    pub fn read_polling(&self, _rcc: &RCC) {
        while (self.rb.isr.read() & (1 << 5)) == 0 {}
        let r = (self.rb.rdr.read() & 0xFF) as u8;
        // TODO use yet-to-define print macro and print r
    }
}

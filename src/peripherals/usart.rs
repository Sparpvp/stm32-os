use core::str;

use volatile_register::RW;

use super::rcc::RCC;

const USART2_ADDR: usize = 0x4000_4400;

pub struct Usart<'a> {
    _rb: &'a mut RegisterBlock,
}

#[repr(C)]
struct RegisterBlock {
    cr1: RW<u32>,
    cr2: RW<u32>,
    cr3: RW<u32>,
    brr: RW<u32>,
    gtpr: RW<u32>,
    rtor: RW<u32>,
    rqr: RW<u32>,
    isr: RW<u32>,
    icr: RW<u32>,
    rdr: RW<u32>,
    tdr: RW<u32>,
}

pub struct UsartConfig {
    pub baud_rate: u32,
    // TODO ...
}

impl<'a> Usart<'a> {
    pub(in crate::peripherals) fn new(c: UsartConfig) -> Usart<'a> {
        let g = unsafe { &mut *(USART2_ADDR as *mut RegisterBlock) };

        unsafe {
            // TODO! I've hardcoded 8mhz as RCC PCLK frequency
            g.brr.write(8_000_000 / c.baud_rate);
            g.cr2.write(0); 
            g.cr3.write(0);
        }

        Usart { _rb: g }
    }

    pub fn write(&self, data: u8, _rcc: &RCC) {
        // Configure USART as transmitter
        unsafe {
            // TE (3): Transmitter Enable
            // UE (0): USART Enable
            self._rb.cr1.write((1 << 3) | (1 << 0));
            // Send byte
            self._rb.tdr.write(data as u32 & 0xFF);
            // Notify end
            self._rb.icr.modify(|m| m | (1 << 6));
        }
    }

    pub fn write_string(&self, s: &str, _rcc: &RCC) {
        unsafe {
            self._rb.cr1.write((1 << 3) | (1 << 0));
            s.as_bytes()
                .into_iter()
                .for_each(|w| self._rb.tdr.write(*w as u32));
            todo!()
        }
    }

    pub fn read(&self, _rcc: &RCC) {
        todo!()
    }
}

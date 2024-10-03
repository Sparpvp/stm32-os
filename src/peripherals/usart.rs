use volatile_register::RW;

use super::rcc::RCC;

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
        let g = unsafe { &mut *(0x4000_4400 as *mut RegisterBlock) };

        // TODO: Initialize...
        unsafe {
            g.brr.write(c.baud_rate);
        }
        todo!();

        Usart { _rb: g }
    }

    pub fn write(_rcc: RCC) {
        todo!()
    }

    pub fn read(_rcc: RCC) {
        todo!()
    }
}

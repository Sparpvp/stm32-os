use volatile_register::RW;

const GPIOA_ADDR: u32 = 0x4800_0000;

pub struct GPIOA<'a> {
    _rb: &'a mut RegisterBlock,
}

#[repr(C)]
pub struct RegisterBlock {
    pub moder: RW<u32>,
    pub otyper: RW<u32>,
    pub ospeedr: RW<u32>,
    pub pupdr: RW<u32>,
    pub idr: RW<u32>,
    pub odr: RW<u32>,
    pub bsrr: RW<u32>,
    pub lckr: RW<u32>,
    pub afrl: RW<u32>,
    pub afrh: RW<u32>,
    pub brr: RW<u32>,
}

impl<'a> GPIOA<'a> {
    pub(in crate::peripherals) fn new() -> GPIOA<'a> {
        let gpioa = unsafe { &mut *(GPIOA_ADDR as *mut RegisterBlock) };
        
        unsafe {
            gpioa.moder.modify(|m| m | (0b10 << 4));
            gpioa.moder.modify(|m| m | (0b10 << 6));

            gpioa.afrl.modify(|m| m | (0b0001 << 8));
            gpioa.afrl.modify(|m| m | (0b0001 << 12));
        };

        GPIOA { _rb: gpioa }
    }
}

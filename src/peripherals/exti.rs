use volatile_register::RW;

const EXTI_ADDR: u32 = 0x4001_0400;
const NVIC_ISER: u32 = 0xE000_E100;

pub struct EXTI {
    _rb: RegisterBlock,
}

#[repr(C)]
pub struct RegisterBlock {
    pub imr: RW<u32>,
    pub emr: RW<u32>,
    pub rtsr: RW<u32>,
    pub ftsr: RW<u32>,
    pub swier: RW<u32>,
    pub pr: RW<u32>,
}

impl EXTI {
    pub fn init() {
        unsafe {
            // Activate USART2 interrupt on NVIC_ISER
            let nvic_iser = &mut *(NVIC_ISER as *mut RW<u32>);
            nvic_iser.modify(|i| i | (1 << 28));

            // Unmask USART2 (line 28) and just to be sure, Mask USART1 (line 27)
            let exti = &mut *(EXTI_ADDR as *mut RegisterBlock);
            exti.imr.modify(|i| (i | (1 << 28)) & !(1 << 27));
        }
    }
}

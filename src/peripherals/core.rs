use core::mem::size_of;

use volatile_register::RW;

const SYST_CSR: usize = 0xE000_E010;
const SYST_RVR: usize = 0xE000_E014;
const IPR_0_BASE: usize = 0xE000_E400;

pub const IT_PENDSV: u8 = 14;

pub struct SysTick;

#[repr(C)]
pub struct IPR {
    pri_n: RW<[u8; 4]>,
}

impl SysTick {
    pub fn enable() {
        let csr = unsafe { &mut *(SYST_CSR as *mut RW<u32>) };
        let rvr = unsafe { &mut *(SYST_RVR as *mut RW<u32>) };

        unsafe {
            // Enable SysTick counter and interrupt generation
            // For completeness sake, it also specifies to use the cpu clock src
            csr.modify(|m| m | (1 << 0) | (1 << 1) | (1 << 2));
            // Here we set the reload value. 24 bits available
            // We'll keep an inside joke value as reload time
            // With 8MHz SYSCLK, its a ~15ms time slice
            rvr.modify(|m| m | 0xBEEF * 3);
        };
    }
}

impl IPR {
    pub fn set_priority(interrupt_num: u8, priority: u8) {
        let ipr_n = interrupt_num / 4;
        let ipr_offset = (interrupt_num % 4) as usize;

        let ipr_reg =
            unsafe { &mut *((IPR_0_BASE + (ipr_n as usize * size_of::<u32>())) as *mut IPR) };
        unsafe {
            ipr_reg.pri_n.modify(|mut m| {
                m[ipr_offset] = priority;
                m
            });
        };
    }
}

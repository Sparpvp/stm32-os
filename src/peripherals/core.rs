use volatile_register::RW;

const SYST_CSR: usize = 0xE000_E010;
const SYST_RVR: usize = 0xE000_E014;
const SYST_CVR: usize = 0xE000_E018;

pub struct SysTick {
    csr: u32,
    rvr: u32,
    cvr: u32,
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
 
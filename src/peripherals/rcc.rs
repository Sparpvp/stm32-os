use volatile_register::RW;

const RCC_ADDR: u32 = 0x4002_1000;

pub struct Rcc<'a> {
    _rb: &'a mut RegisterBlock,
}

#[repr(C)]
struct RegisterBlock {
    pub cr: RW<u32>,
    pub cfgr: RW<u32>,
    pub cir: RW<u32>,
    pub apb2rstr: RW<u32>,
    pub apb1rstr: RW<u32>,
    pub ahbenr: RW<u32>,
    pub apb2enr: RW<u32>,
    pub apb1enr: RW<u32>,
    pub bdcr: RW<u32>,
    pub csr: RW<u32>,
    pub ahbrstr: RW<u32>,
    pub cfgr2: RW<u32>,
    pub cfgr3: RW<u32>,
    pub cr2: RW<u32>,
}

#[repr(C)]
struct ROBlock {
    cr: u32,
    cfgr: u32,
    cir: u32,
    apb2rstr: u32,
    apb1rstr: u32,
    ahbenr: u32,
    apb2enr: u32,
    apb1enr: u32,
    bdcr: u32,
    csr: u32,
    ahbrstr: u32,
    cfgr2: u32,
    cfgr3: u32,
    cr2: u32,
}

pub struct RccConfig {
    pub sysclk: u32,
    pub pclk: u32,
}

// Factory pattern: construct RCC only if Rcc has been acquired.
// => Pass RCC as a proof of configured clocks
pub struct RCC {
    _rb: ROBlock,
}

impl<'a> Rcc<'a> {
    pub fn new(c: RccConfig) -> Rcc<'a> {
        let rcc = unsafe { &mut *(RCC_ADDR as *mut RegisterBlock) };

        unsafe {
            // Modify default clocks...
            // TODO: Use config to do so

            // Enable GPIOA clock
            rcc.ahbenr.modify(|m| m | (1 << 17));
            // Enable USART2 clock
            rcc.apb1enr.modify(|m| m | (1 << 17));
        };

        Rcc { _rb: rcc }
    }

    pub(in crate::peripherals) fn freeze(self) -> RCC {
        let rb = ROBlock {
            cr: self._rb.cr.read(),
            cfgr: self._rb.cfgr.read(),
            cir: self._rb.cir.read(),
            apb2rstr: self._rb.apb2rstr.read(),
            apb1rstr: self._rb.apb1rstr.read(),
            ahbenr: self._rb.ahbenr.read(),
            apb2enr: self._rb.apb2enr.read(),
            apb1enr: self._rb.apb1enr.read(),
            bdcr: self._rb.bdcr.read(),
            csr: self._rb.csr.read(),
            ahbrstr: self._rb.ahbrstr.read(),
            cfgr2: self._rb.cfgr2.read(),
            cfgr3: self._rb.cfgr3.read(),
            cr2: self._rb.cr2.read(),
        };

        RCC { _rb: rb }
    }
}

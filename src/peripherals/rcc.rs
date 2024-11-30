use volatile_register::RW;

const RCC_ADDR: u32 = 0x4002_1000;
const HSI_DEFAULT_CLK: u32 = 8_000_000;

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

// Construct RCC only if Rcc has been acquired.
// => Pass RCC as a proof of configured clocks
pub struct RCC {
    _rb: ROBlock,
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
    pub source: ClockSource,
    pub sysclk: SysClkMultiplier,
    pub pclk: PPREScaler,
}

#[derive(PartialEq)]
pub enum ClockSource {
    HSI,
    PLL,
}

pub struct SysClkMultiplier(u32);
impl SysClkMultiplier {
    pub const _HSI_DEFAULT: Self = Self(HSI_DEFAULT_CLK);
    pub const PLL_MUL2: Self = Self(0b0000);
    pub const PLL_MUL3: Self = Self(0b0001);
    pub const PLL_MUL4: Self = Self(0b0010);
    pub const PLL_MUL5: Self = Self(0b0011);
    pub const PLL_MUL6: Self = Self(0b0100);
}

pub struct PPREScaler(u32);
impl PPREScaler {
    pub const AS_SYSCLK: Self = Self(0b000);
    pub const PPRE_DIV2: Self = Self(0b100);
    pub const PPRE_DIV4: Self = Self(0b101);
    pub const PPRE_DIV8: Self = Self(0b110);
    pub const PPRE_DIV16: Self = Self(0b111);
}

impl<'a> Rcc<'a> {
    pub fn new(c: RccConfig) -> Rcc<'a> {
        let rcc = unsafe { &mut *(RCC_ADDR as *mut RegisterBlock) };

        unsafe {
            if c.source == ClockSource::PLL {
                // Modify default clocks
                // 1. Disable the PLL
                rcc.cr.modify(|m| m & !(1 << 24));
                // 2. Wait until the PLL is fully stopped
                while (rcc.cr.read() & (1 << 25)) == 1 {}
                // 3. Let's setup the clock
                rcc.cfgr.modify(|g| (g & !(0b1111 << 18)) | c.sysclk.0);
                // 4. Enable the PLL
                rcc.cr.modify(|m| m | (1 << 24));
                // 5. Wait until the PLL is fully started again
                while (rcc.cr.read() & (1 << 25)) == 1 {}

                rcc.cfgr.modify(|m| (m & !(0b11)) | 0b10);
            }

            if c.pclk.0 != PPREScaler::AS_SYSCLK.0 {
                rcc.cfgr.modify(|m| (m & !(0b111 << 8)) | c.pclk.0);
            }

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

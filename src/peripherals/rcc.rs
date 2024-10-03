pub struct Rcc {}

impl Rcc {
    pub(in crate::peripherals) fn new() -> Rcc {
        todo!()
    }
}

// Factory pattern: construct RCC only if Rcc has been acquired.
// => Pass RCC as a proof of configured clocks
pub struct RCC {}

impl Rcc {
    pub fn freeze() -> RCC {
        todo!()
    }
}

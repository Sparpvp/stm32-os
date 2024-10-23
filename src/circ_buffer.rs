use cortex_m_semihosting::hprintln;

use crate::peripherals::exti;

const CAP: usize = 30;

pub struct CircularBuffer {
    buf: [u8; CAP],
    read_index: usize,
    write_index: usize,
}

#[derive(Clone)]
pub struct EmptyBufferError;

pub static mut G_BUFFER: Option<CircularBuffer> = None;

impl CircularBuffer {
    pub fn init() {
        assert_eq!(unsafe { G_BUFFER.is_none() }, true);

        let cb = CircularBuffer {
            buf: [0; CAP],
            read_index: 0,
            write_index: 0,
        };
        unsafe {
            G_BUFFER.replace(cb);
        };
    }

    pub fn put(data: u8) {
        assert_eq!(unsafe { G_BUFFER.is_some() }, true);

        let mut cb = unsafe { G_BUFFER.take().unwrap() };
        if (cb.write_index + 1) % CAP == cb.read_index {
            // Mask EXTI line 28
            exti::EXTI::mask_usart2();
        }
        cb.buf[cb.write_index] = data;
        cb.write_index = (cb.write_index + 1) % CAP;

        unsafe {
            G_BUFFER.replace(cb);
        };
    }

    pub fn get() -> Result<u8, EmptyBufferError> {
        assert_eq!(unsafe { G_BUFFER.is_some() }, true);

        let mut cb = unsafe { G_BUFFER.take().unwrap() };
        let res = match cb.read_index != cb.write_index {
            true => {
                if cb.read_index == (cb.write_index - 1) % CAP {
                    exti::EXTI::unmask_usart2();
                }

                let data = cb.buf[cb.read_index];
                cb.read_index = (cb.read_index + 1) % CAP;

                Ok(data)
            }
            false => Err(EmptyBufferError),
        };

        unsafe {
            G_BUFFER.replace(cb);
        };

        res
    }
}

use crate::{circ_buffer::CircularBuffer, peripherals::usart::G_USART};

use super::critical_section::critical_section;

// Clears USART interrupt
pub(in crate::trap) fn usart2_transmission_complete() {
    unsafe {
        let usart = G_USART.as_ref().unwrap();
        usart.rb.icr.modify(|m| m | (1 << 6));
    };
}

pub(in crate::trap) fn usart2_irq_receive() {
    let usart = unsafe { G_USART.take().unwrap() };
    let data = (usart.rb.rdr.read() & 0xFF) as u8;
    unsafe {
        G_USART.replace(usart);
    };

    // Append data into the circular buffer
    // Note on safety: in this context put is alredy in a critical section

    critical_section(|c| {
        CircularBuffer::put(data, c);
    })
}

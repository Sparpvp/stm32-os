use crate::peripherals::usart::G_USART;

pub fn usart2_irq_receive() {
    let usart = unsafe { G_USART.take().unwrap() };
    let data = (usart.rb.rdr.read() & 0xFF) as u8;
    unsafe {
        G_USART.replace(usart);
    };

    // Append data into the circular buffer

    todo!();
}

use crate::{circ_buffer::CircularBuffer, trap::critical_section::critical_section};

pub fn shell() {
    // TODO: Get from circular buffer till enter is received
    // Then process commands
    // let mut curr_command: Vec<char> = Vec::new();

    loop {
        match critical_section(CircularBuffer::get) {
            Ok(cb) if cb as char != ' ' => {
                // TODO handle character
            }
            Ok(cb) if cb as char == ' ' => break,
            Ok(_) => unreachable!(),
            Err(_) => {
                // TODO yield syscall
            }
        }
    }

    loop {}
}

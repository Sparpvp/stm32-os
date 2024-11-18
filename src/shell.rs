use alloc::vec::Vec;

use crate::{circ_buffer::CircularBuffer, trap::critical_section::critical_section};
use crate::{print, println};

pub fn shell() {
    // TODO: Get from circular buffer till enter is received
    // Then process commands

    critical_section(|_c| {
        let mut curr_command: Vec<char> = Vec::new();
        curr_command.push('a');
        curr_command.push('b');
        curr_command.push('c');
        curr_command.push('d');

        let b: Vec<char> = curr_command
            .into_iter()
            .map(|i| (i as u8 + 1) as char)
            .collect();
        b.into_iter().for_each(|i| println!("Test {}", i));
    });
    let mut curr_command: Vec<char> = Vec::new();

    loop {
        match critical_section(CircularBuffer::get) {
            Ok(cb) if cb as char != ' ' => {
                // TODO handle character
                critical_section(|_c| {
                    curr_command.push(cb as char);
                    println!("TEST {}", cb as char);
                })
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

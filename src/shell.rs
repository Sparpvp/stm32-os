use alloc::vec::Vec;

use crate::println;
use crate::{circ_buffer::CircularBuffer, trap::critical_section::critical_section};

fn process_command(cmd: Vec<char>) {
    todo!()
}

pub fn shell() {
    // critical_section(|_c| {
    //     let mut curr_command: Vec<char> = Vec::new();
    //     curr_command.push('a');
    //     curr_command.push('b');
    //     curr_command.push('c');
    //     curr_command.push('d');

    //     let b: Vec<char> = curr_command
    //         .into_iter()
    //         .map(|i| (i as u8 + 1) as char)
    //         .collect();
    //     b.into_iter().for_each(|i| println!("Test {}", i));
    // });

    loop {
        let mut curr_command: Vec<char> = Vec::new();

        loop {
            match critical_section(CircularBuffer::get) {
                Ok(cb) if cb as char != ' ' => critical_section(|_c| {
                    curr_command.push(cb as char);
                    println!("read: {}", cb as char);
                }),
                Ok(cb) if cb as char == ' ' => break,
                Ok(_) => unreachable!(),
                Err(_) => {
                    // TODO yield syscall
                }
            }
        }

        process_command(curr_command);
    }
}

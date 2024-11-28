pub mod commands;
pub mod parser;

use crate::{circ_buffer::CircularBuffer, trap::critical_section::critical_section};
use alloc::{string::String, vec::Vec};
use parser::process_command;

pub enum ShellError {
    ParserError(String),
    ExecutionError,
}

pub fn shell() {
    loop {
        let mut curr_command: Vec<char> = Vec::new();

        loop {
            match critical_section(CircularBuffer::get) {
                Ok(cb) if cb as char != '\r' => critical_section(|_c| {
                    curr_command.push(cb as char);
                    print!("{}", cb as char)
                }),
                Ok(cb) if cb as char == '\r' => {
                    print!("\r\n");
                    break;
                }
                Ok(_) => unreachable!(),
                Err(_) => {
                    // TODO yield syscall
                }
            }
        }

        match process_command(curr_command) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
}

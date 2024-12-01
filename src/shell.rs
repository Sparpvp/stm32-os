pub mod commands;
pub mod parser;

use core::fmt::Display;

use crate::{
    circ_buffer::CircularBuffer,
    syscall::{syscall, SVCallId},
    trap::critical_section::critical_section,
};
use alloc::{string::String, vec::Vec};
use parser::process_command;

#[derive(Debug)]
pub enum ShellError {
    ParserError(String),
    ExecutionError,
}

impl Display for ShellError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn shell() {
    loop {
        let mut curr_command: Vec<char> = Vec::new();

        loop {
            match critical_section(CircularBuffer::get) {
                Ok(cb) if cb as char != '\r' => critical_section(|_c| {
                    critical_section(|_cs| {
                        curr_command.push(cb as char);
                    });
                    print!("{}", cb as char)
                }),
                Ok(cb) if cb as char == '\r' => {
                    print!("\r\n");
                    break;
                }
                Ok(_) => unreachable!(),
                Err(_) => unsafe {
                    syscall(SVCallId::Yield);
                },
            }
        }

        match process_command(curr_command) {
            Ok(_) => {}
            Err(e) => println!("unable to execute command. err: {}", e),
        };
    }
}

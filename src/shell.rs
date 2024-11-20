use alloc::string::String;
use alloc::vec::Vec;

use crate::println;
use crate::{circ_buffer::CircularBuffer, trap::critical_section::critical_section};

// Consider using dynamic-dispatched errors. A bit heavy for our potato uC.
struct ParserError;

fn rm_proc_by_pid(pid: u16) -> Result<(), ParserError> {
    Ok(())
}
fn rm_proc_by_name(name: String) -> Result<(), ParserError> {
    Ok(())
}

fn add_proc() -> Result<(), ParserError> {
    Ok(())
}

fn process_command(cmd: Vec<char>) -> Result<(), ParserError> {
    let mut args: Vec<char> = Vec::new();
    let mut opcode: Vec<char> = Vec::with_capacity(5);
    let mut arg_start = false;
    for e in cmd {
        if e.is_whitespace() && !arg_start {
            arg_start = true;
            continue;
        } else if !e.is_whitespace() {
            opcode.push(e);
        }
        if arg_start {
            args.push(e);
        }
    }

    let ret = match opcode.iter().collect::<String>().as_str() {
        "rmproc" => {
            todo!();
        }
        "addproc" => {
            todo!();
        }
        _ => {
            todo!();
            Err(ParserError)
        }
    };

    ret
}

pub fn shell() {
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

        match process_command(curr_command) {
            Ok(_) => todo!(),
            Err(_) => todo!(),
        };
    }
}

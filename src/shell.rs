use alloc::string::String;
use alloc::vec::Vec;

use crate::trap::critical_section::CriticalSection;
use crate::{circ_buffer::CircularBuffer, trap::critical_section::critical_section};

// Consider using dynamic-dispatched errors. A bit heavy for our potato uC.
struct ParserError;

fn rm_proc_by_pid(pid: u16, _cs: &CriticalSection) -> Result<(), ParserError> {
    Ok(())
}
fn rm_proc_by_name(name: String, _cs: &CriticalSection) -> Result<(), ParserError> {
    Ok(())
}

fn add_proc(_cs: CriticalSection) -> Result<(), ParserError> {
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

    let opcode_bind = opcode.iter().collect::<String>();
    let opcode = opcode_bind.as_str();

    let res = match opcode {
        "rmproc" => {
            let r = args
                .iter()
                .position(|&c| c == ' ')
                .and_then(|p| {
                    let args = args.drain(..=p).collect::<String>();
                    args.parse::<u16>()
                        .ok()
                        .and_then(|p| critical_section(|cs| rm_proc_by_pid(p, cs)).ok())
                        .ok_or_else(|| ParserError)
                        .or_else(|_e| critical_section(|cs| rm_proc_by_name(args, cs)))
                        .ok()
                })
                .ok_or_else(|| ParserError);
            r
        }
        "addproc" => {
            todo!();
        }
        _ => Err(ParserError), // Unrecognized opcode
    };

    res
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
                    print!("\n");
                    break;
                }
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

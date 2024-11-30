use alloc::{string::String, vec::Vec};

use crate::{shell::commands::add_proc, trap::critical_section::critical_section};

use super::{
    commands::{rm_proc_by_name, rm_proc_by_pid},
    ShellError,
};

pub(in crate::shell) fn process_command(cmd: Vec<char>) -> Result<(), ShellError> {
    let mut args: Vec<char> = Vec::new();
    let mut opcode: Vec<char> = Vec::with_capacity(5);
    let mut arg_start = false;
    for e in cmd {
        if e.is_whitespace() && !arg_start {
            arg_start = true;
            continue;
        } else if !e.is_whitespace() && !arg_start {
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
                .collect::<String>()
                .parse::<u16>()
                .ok()
                .and_then(|p| critical_section(|cs| rm_proc_by_pid(p, cs)).ok())
                .ok_or_else(|| ShellError::ExecutionError)
                .or_else(|_e| {
                    let to_name = args.iter().collect::<String>();
                    critical_section(|cs| rm_proc_by_name(to_name, cs))
                });
            r
        }
        "addproc" => {
            let s: &str = &args.iter().collect::<String>();
            critical_section(|cs| add_proc(cs, s))
        }
        _ => Err(ShellError::ParserError(String::from("Unrecognized opcode"))),
    };

    res
}

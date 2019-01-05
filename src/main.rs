//! > a custom command extending `cargo`
//!
//! This program is intended to print the memory usage of a cargo binary. If a
//! file `memory.x` is found in the crate root, this file is taken into account
//! in order to calculate the percentage of used memory.
//!
//! # Usage
//! Executing `cargo size` in a crate (root or a subdirectory) with a file
//! `memory.x` present in the crate root builds the development binary and
//! prints an output similar to:
//! ```bash
//! $ cargo size
//! Memory Usage
//! ------------
//! Program:    7420 bytes (23.4% full)
//! Data:          8 bytes (1.2% full)
//! ```
//!
//! The command `cargo size --release` does the same, but builds the release
//! binary if necessary and prints its size.
//!
//! If the file `memory.x` is not found the percentages are omitted.
extern crate colored;
extern crate elf;
extern crate ldscript_parser;

use crate::error::Error;
use crate::mode::Mode;
use colored::Colorize;
use ldscript_parser::RootItem::Memory;
use std::env;
use std::fs;
use std::process;

mod binary;
mod cargo;
mod error;
mod mode;

/// Try to execute the whole program or return at the first error.
///
/// On success, the function returns the program output.
fn try_main() -> Result<String, Error> {
    let mode = Mode::new();

    change_directory()?;
    mode.build_binary()?;
    let binary = mode.binary()?;
    let (code, data) = binary::read_size_from(&binary)?;

    if let Some((code_memory, data_memory)) = memory_size() {
        let code_percentage = code as f32 / code_memory as f32 * 100.0;
        let data_percentage = data as f32 / data_memory as f32 * 100.0;
        Ok(format!(
            "Memory Usage
             ------------
             Program: {:>7} bytes ({:.1}% full)
             Data:    {:>7} bytes ({:.1}% full)",
            code, code_percentage, data, data_percentage
        ))
    } else {
        Ok(format!(
            "Memory Usage
             ------------
             Program: {:>7} bytes
             Data:    {:>7} bytes",
            code, data
        ))
    }
}

/// Changes the current working directory to the crate root if possible.
fn change_directory() -> Result<(), Error> {
    env::set_current_dir(cargo::root()?)?;

    Ok(())
}

/// Read the file `memory.x` if present and return the program and data memory
/// size.
///
/// If the file does not exist or has an invalid format, `None` is returned. To
/// be valid, there have to be two sections present in the memory section, which
/// are named `flash` and `ram` (case is ignored).
fn memory_size() -> Option<(u64, u64)> {
    fs::read_to_string("memory.x")
        .ok()
        .and_then(|content| ldscript_parser::parse(&content).ok())
        .and_then(|items| {
            for item in items {
                match item {
                    Memory { regions } => return Some(regions),
                    _ => {}
                }
            }
            None
        })
        .and_then(|sections| {
            let mut code = 0;
            let mut data = 0;
            for section in sections {
                if section.name.to_lowercase() == "flash" {
                    code += section.length;
                }
                if section.name.to_lowercase() == "ram" {
                    data += section.length;
                }
            }

            if code != 0 && data != 0 {
                Some((code, data))
            } else {
                None
            }
        })
}

/// The program entry point.
///
/// This function executes `try_main()` and prints its result state. In case of
/// success, the output is written to `stdout` with `Printing` in front of it to
/// mimic the usual cargo behavior. The program exits with the status code `0`.
///
/// In case of an error, the error is printed to `stderr` prefixed with a red
/// _Error_ in front of it to mimic the cargo behavior. The program exits with
/// the status code `1`.
fn main() {
    match try_main() {
        Ok(output) => {
            println!("{:>12} {}", "Printing".bright_green().bold(), output);
            process::exit(0);
        }
        Err(e) => {
            eprintln!("{:>12} {}", "Error".bright_red().bold(), e);
            process::exit(1);
        }
    }
}

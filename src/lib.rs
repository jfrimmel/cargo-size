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
use ldscript_parser::RootItem::Memory;
use std::env;
use std::fs;

pub mod binary;
pub mod cargo;
pub mod error;
pub mod mode;

/// Changes the current working directory to the crate root if possible.
pub fn change_directory() -> Result<(), Error> {
    env::set_current_dir(cargo::root()?)?;

    Ok(())
}

/// Read the file `memory.x` if present and return the program and data memory
/// size.
///
/// If the file does not exist or has an invalid format, `None` is returned. To
/// be valid, there have to be two sections present in the memory section, which
/// are named `flash` and `ram` (case is ignored).
pub fn memory_size() -> Option<(u64, u64)> {
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

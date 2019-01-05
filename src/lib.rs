//! > a custom command extending `cargo`
//!
//! This program is intended to print the memory usage of a cargo binary. If a
//! file `memory.x` is found in the crate root, this file is taken into account
//! in order to calculate the percentage of used memory.
//!
//! # Dependencies
//! - Cargo is required for building the project.
//!
//! # Usage
//! Executing `cargo size` in a crate (root or a subdirectory) with a file
//! `memory.x` present in the crate root builds the development binary and
//! prints an output similar to:
//! ```bash
//! $ cargo size
//!    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
//!    Printing Memory Usage
//!             ------------
//!             Program:   55652 bytes (42.5% full)
//!             Data:          8 bytes (0.0% full)
//! ```
//!
//! The command `cargo size --release` does the same, but builds the release
//! binary if necessary and prints its size.
//!
//! If the file `memory.x` is not found or is invalid the percentages are
//! omitted, without any error or warning message:
//! ```bash
//! $ cargo size
//!    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
//!    Printing Memory Usage
//!             ------------
//!             Program: 1486351 bytes
//!             Data:       4656 bytes
//! ```
//! The file `memory.x` has to contain a `MEMORY` directive with at least the
//! two sections `flash` and `ram` (case is ignored). The size of those two
//! memories is used to calculate the percentages.
//!
//! If the binary (either the development or the release one, as specified) is
//! not up-to-date, cargo is used to build it.
//!
//! # Errors
//! If any error is detected, the application emits an error message and exits
//! with the exit code `1` to indicate failure.
//!
//! Possible errors include:
//! - The program was started in a directory, which is not a cargo project
//!   (subdirectories of such a project are perfectly fine).
//! - The manifest file `Cargo.toml` is present, but could not be parsed, e.g.
//!   the crate name is missing.
//! - The build command failed, e.g. the code contains an error.
//! - The binary was built successfully, but could not be found in the target
//!   directory. This is most likely if you are cross-compiling code for another
//!   platform. Then the binary is located in a subdirectory named as the
//!   target. The application tries some known platform, but if yours is not
//!   known yet, the command will fail.
//! - The binary has an invalid format, e.g. it is not an ELF-file or it is
//!   corrupt.
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

/// Changes the current working directory to the crate root.
///
/// # Errors
/// The function can fail for two reasons: either the project is not a cargo
/// project or an I/O error occurred while reading the project or setting the
/// current working directory.
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

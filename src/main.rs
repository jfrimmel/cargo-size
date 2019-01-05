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
//! Program:    7420 bytes (23.4% Full)
//! Data:          8 bytes (1.2% Full)
//! ```
//!
//! The command `cargo size --release` does the same, but builds the release
//! binary if necessary and prints its size.
//!
//! If the file `memory.x` is not found the percentages are omitted.
extern crate colored;

use crate::error::Error;
use crate::mode::Mode;
use colored::Colorize;
use std::env;
use std::process;

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

    Ok(format!("{:?}", mode.binary()?))
}

/// Changes the current working directory to the crate root if possible.
fn change_directory() -> Result<(), Error> {
    env::set_current_dir(cargo::root()?)?;

    Ok(())
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

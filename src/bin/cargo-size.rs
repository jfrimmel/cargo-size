//! The actual binary, that uses the rust library.
extern crate cargo_size;

use cargo_size::binary;
use cargo_size::change_directory;
use cargo_size::error::Error;
use cargo_size::memory_size;
use cargo_size::mode::Mode;
use colored::Colorize;
use std::env;
use std::process;

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
    let name = option_env!("CARGO_PKG_NAME").unwrap_or("cargo-size");
    if env::args().any(|arg| arg == "--help" || arg == "-h") {
        println!(
            "{}
A command extending cargo to print the memory usage of a program

USAGE:
    cargo size [OPTIONS]

OPTIONS:
      --release               Print the size of the release binary
                              (debug if flag is not present)
      --help                  Print this help screen and exit
      --version               Print the version number and exit",
            name
        );

        process::exit(0);
    } else if env::args().any(|arg| arg == "--version" || arg == "-v") {
        let version = option_env!("CARGO_PKG_VERSION").unwrap_or("?.?.?");
        println!("{} {}", name, version);

        process::exit(0);
    }

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

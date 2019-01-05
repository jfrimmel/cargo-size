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

fn main() {
    unimplemented!();
}

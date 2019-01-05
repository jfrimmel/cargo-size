# cargo-size
> a custom command extending `cargo` to print the memory usage

# Dependencies
- `cargo` is required for building the crate you want to display the size of.

# Usage
Executing `cargo size` in a crate (root or a subdirectory) with a file `memory.x` present in the crate root builds the development binary and prints an output similar to:
```
$ cargo size
   Finished dev [unoptimized + debuginfo] target(s) in 0.37s
   Printing Memory Usage
            ------------
            Program:   55652 bytes (42.5% full)
            Data:          8 bytes (0.0% full)
```
The command `cargo size --release` does the same, but builds the release binary if necessary and prints its size.

If the file `memory.x` is not found or is invalid the percentages are omitted, without any error or warning message:
```
$ cargo size
   Finished dev [unoptimized + debuginfo] target(s) in 0.01s
   Printing Memory Usage
            ------------
            Program: 1486351 bytes
            Data:       4656 bytes
```
The file `memory.x` has to contain a `MEMORY` directive with at least the two sections `flash` and `ram` (case is ignored). The size of those two memories is used to calculate the percentages.

If the binary (either the development or the release one, as specified) is not up-to-date, cargo is used to build it.

# Errors
If any error is detected, the application emits an error message and exits with the exit code `1` to indicate failure.

Possible errors include:
- The program was started in a directory, which is not a cargo project (subdirectories of such a project are perfectly fine).
- The manifest file `Cargo.toml` is present, but could not be parsed, e.g. the crate name is missing.
- The build command failed, e.g. the code contains an error.
- The binary was built successfully, but could not be found in the target directory. This is most likely if you are cross-compiling code for another platform. Then the binary is located in a subdirectory named as the target. The application tries some known platform, but if yours is not known yet, the command will fail.
- The binary has an invalid format, e.g. it is not an ELF-file or it is corrupt.


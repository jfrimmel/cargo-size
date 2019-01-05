//! A module containing application mode dependent functionality.
use crate::cargo;
use crate::error::Error;
use std::env;
use std::path::PathBuf;
use std::process::Command;

/// The supported cross platform targets.
///
/// If compiling for a cross platform, the binary is searched in these
/// directories. The first match is used.
pub const SUPPORTED_CROSS_PLATFORMS: [&str; 4] = [
    "thumbv6m-none-eabi",
    "thumbv7m-none-eabi",
    "thumbv7em-none-eabi",
    "thumbv7em-none-eabihf",
];

/// The mode of the tool (debug or release).
pub enum Mode {
    Debug,
    Release,
}
impl Mode {
    /// Create a new application mode depending on the presence of the
    /// `--release` flag.
    pub fn new() -> Mode {
        if env::args().find(|arg| arg == "--release").is_some() {
            Mode::Release
        } else {
            Mode::Debug
        }
    }

    /// Build the binary of the crate.
    pub fn build_binary(&self) -> Result<(), Error> {
        let status = match self {
            Mode::Debug => Command::new("cargo").arg("build").status()?,
            Mode::Release => Command::new("cargo")
                .args(&["build", "--release"])
                .status()?,
        };
        if !status.success() {
            Err(Error::BuildError)
        } else {
            Ok(())
        }
    }

    /// Query the path to the binary binary.
    ///
    /// Dependent of the mode the method searches for the binary in the `debug`
    /// or `release` folder. If the binary is not in that folder, some other
    /// folders are searched. This may occur in cross-platform builds. Currently
    /// only ARM targets are supported.
    ///
    /// If the binary can not be found a `BinaryNotFound` error is returned.
    pub fn binary(&self) -> Result<PathBuf, Error> {
        let target_dir = env::current_dir()?.join("target");
        let name = cargo::crate_name()?;

        target_dir
            .read_dir()?
            .filter_map(|entry| entry.ok())
            .find(|entry| match self {
                Mode::Debug => entry.file_name() == "debug",
                Mode::Release => entry.file_name() == "release",
            })
            .map(|entry| entry.path().join(&name))
            .filter(|entry| entry.exists())
            .or({
                target_dir
                    .read_dir()?
                    .filter_map(|entry| entry.ok())
                    .find(|entry| {
                        SUPPORTED_CROSS_PLATFORMS
                            .contains(&entry.file_name().to_str().unwrap())
                    })
                    .map(|entry| entry.path())
                    .map(|entry| {
                        entry.join(match self {
                            Mode::Debug => "debug",
                            Mode::Release => "release",
                        })
                    })
                    .map(|entry| entry.join(&name))
                    .filter(|entry| entry.exists())
            })
            .ok_or(Error::BinaryNotFound)
    }
}

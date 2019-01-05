//! A module containing the `Error` type and its implementations.
use std::fmt::{self, Display, Formatter};
use std::io;

/// Reasons for program failure.
#[derive(Debug)]
pub enum Error {
    /// The program was not executed inside a cargo project.
    NotACrate,
    /// The crate has an invalid `Cargo.toml` file.
    InvalidManifest,
    /// An I/O error
    IoError(io::Error),
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::NotACrate => write!(f, "Not a cargo project, aborting."),
            Error::InvalidManifest => {
                write!(f, "Cargo.toml is invalid, aborting.")
            }
            Error::IoError(e) => write!(f, "I/O error ({})", e),
        }
    }
}
impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IoError(e)
    }
}
impl PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        match (self, other) {
            (Error::NotACrate, Error::NotACrate) => true,
            (Error::IoError(_), Error::IoError(_)) => true,
            _ => false,
        }
    }
}

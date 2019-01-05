//! A module containing the `Error` type and its implementations.
use std::fmt::{self, Display, Formatter};

/// Reasons for program failure.
pub enum Error {
    /// The program was not executed inside a cargo project.
    NotACrate,
}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Error::NotACrate => write!(f, "Not a cargo project, aborting."),
        }
    }
}

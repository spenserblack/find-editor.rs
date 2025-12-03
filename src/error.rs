//! Module for this crate's error type.
use super::{ParseError, WhichError};
use std::error::Error as ErrorTrait;
use std::fmt::{self, Display};
use std::io;

/// Possible error type returned by 3rd-party tools.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// A command failed to start.
    #[cfg(feature = "open")]
    Io(io::Error),
    /// An error returned when failing to split shell words using
    /// [`shell-words`](https://crates.io/crates/shell-words).
    #[cfg(feature = "split")]
    ShellWords(ParseError),
    /// An error returned when failing to find a command with
    /// [`which`](https://crates.io/crates/which).
    #[cfg(feature = "which")]
    Which(WhichError),
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            #[cfg(feature = "open")]
            Self::Io(e) => Display::fmt(e, f),
            #[cfg(feature = "split")]
            Self::ShellWords(e) => Display::fmt(e, f),
            #[cfg(feature = "which")]
            Self::Which(e) => Display::fmt(e, f),
        }
    }
}

impl ErrorTrait for Error {}

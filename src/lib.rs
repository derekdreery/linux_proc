//! Parsers for the contents of the `/proc` directory.
//!

pub mod diskstats;
pub mod stat;
pub mod uptime;
mod util;

use std::fmt;

/// A very simple error handler.
pub struct Error(String);

impl From<String> for Error {
    fn from(f: String) -> Error {
        Error(f)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(f: &str) -> Error {
        Error(f.into())
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl std::error::Error for Error {}

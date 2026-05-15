//! Errors returned by the `uti` crate.

use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UTIError {
    /// Caller passed an invalid string (NUL byte) to a constructor.
    InvalidArgument(String),
    /// No `UTType` matches the requested identifier / extension / MIME.
    NotFound(String),
}

impl fmt::Display for UTIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(m) => write!(f, "invalid argument: {m}"),
            Self::NotFound(m) => write!(f, "no UTType matches: {m}"),
        }
    }
}

impl std::error::Error for UTIError {}

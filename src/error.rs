//! Errors returned by the `uti` crate.

use core::fmt;

/// Errors returned by `UTI` lookups, conversions, and bridge operations.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum UTIError {
    /// Caller passed an invalid string (NUL byte) to a constructor.
    InvalidArgument(String),
    /// No `UTType` matches the requested identifier / extension / MIME.
    NotFound(String),
    /// The framework reported a runtime error while performing an operation.
    OperationFailed(String),
}

/// Formats `UTIError` values for user-facing messages.
impl fmt::Display for UTIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidArgument(m) => write!(f, "invalid argument: {m}"),
            Self::NotFound(m) => write!(f, "no UTType matches: {m}"),
            Self::OperationFailed(m) => write!(f, "operation failed: {m}"),
        }
    }
}

/// Exposes `UTIError` through Rust's standard error trait.
impl std::error::Error for UTIError {}

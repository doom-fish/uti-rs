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

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn invalid_argument_display_is_user_facing() {
        assert_eq!(
            UTIError::InvalidArgument("bad argument".to_owned()).to_string(),
            "invalid argument: bad argument"
        );
    }

    #[test]
    fn not_found_display_is_user_facing() {
        assert_eq!(
            UTIError::NotFound("public.png".to_owned()).to_string(),
            "no UTType matches: public.png"
        );
    }

    #[test]
    fn operation_failed_display_is_user_facing() {
        assert_eq!(
            UTIError::OperationFailed("bridge down".to_owned()).to_string(),
            "operation failed: bridge down"
        );
    }

    #[test]
    fn debug_clone_and_source_behave_as_expected() {
        let err = UTIError::NotFound("public.png".to_owned());
        assert_eq!(err.clone(), err);
        assert_eq!(format!("{err:?}"), "NotFound(\"public.png\")");
        assert!(err.source().is_none());
    }
}

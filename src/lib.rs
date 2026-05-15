#![doc = include_str!("../README.md")]
//!
//! ---
//!
//! # API documentation
//!
//! Safe Rust bindings for Apple's
//! [UniformTypeIdentifiers](https://developer.apple.com/documentation/uniformtypeidentifiers)
//! framework on macOS — file-type and MIME identification via `UTType`.

#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod error;
pub mod ffi;
pub mod uttype;

pub use error::UTIError;
pub use uttype::UTI;

/// Common imports.
pub mod prelude {
    pub use crate::error::UTIError;
    pub use crate::uttype::UTI;
}

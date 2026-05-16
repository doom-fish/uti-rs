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

pub mod additions;
pub mod core_types;
pub mod error;
pub mod ffi;
pub mod item_provider;
pub mod os_type;
pub mod tag_class;
mod util;
pub mod uttype;

pub use error::UTIError;
pub use item_provider::{ItemProvider, LoadedFileRepresentation, RepresentationVisibility};
pub use uttype::{UTI, UTType, UTTypeReference};

/// Common imports.
pub mod prelude {
    pub use crate::additions;
    pub use crate::core_types;
    pub use crate::error::UTIError;
    pub use crate::item_provider::{
        ItemProvider, LoadedFileRepresentation, RepresentationVisibility,
    };
    pub use crate::os_type;
    pub use crate::tag_class;
    pub use crate::uttype::{UTI, UTType, UTTypeReference};
}

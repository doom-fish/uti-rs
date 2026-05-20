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
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_api;
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
pub use uttype::{UTType, UTTypeReference, UTI};

/// Common imports.
pub mod prelude {
    pub use crate::additions;
    #[cfg(feature = "async")]
    pub use crate::async_api::AsyncItemProvider;
    pub use crate::core_types;
    pub use crate::error::UTIError;
    pub use crate::item_provider::{
        ItemProvider, LoadedFileRepresentation, RepresentationVisibility,
    };
    pub use crate::os_type;
    pub use crate::tag_class;
    pub use crate::uttype::{UTType, UTTypeReference, UTI};
}

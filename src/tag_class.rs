//! Tag-class constants used by [`UTI::from_tag`](crate::UTI::from_tag)
//! and [`UTI::types_with_tag`](crate::UTI::types_with_tag).

/// Filename-extension tag class (e.g. `"png"`).
pub const FILENAME_EXTENSION: &str = "public.filename-extension";

/// MIME-type tag class (e.g. `"image/png"`).
pub const MIME_TYPE: &str = "public.mime-type";

/// `OSType` / `FourCharCode` tag class (e.g. `"PNGf"`).
///
/// Apple does not publish this helper in `UTTagClass.h`, but live type-tag
/// dictionaries use the raw key `com.apple.ostype` for classic four-character
/// Macintosh file-type codes.
pub const OS_TYPE: &str = "com.apple.ostype";

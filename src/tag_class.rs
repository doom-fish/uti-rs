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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filename_extension_matches_expected_value() {
        assert_eq!(FILENAME_EXTENSION, "public.filename-extension");
    }

    #[test]
    fn mime_type_matches_expected_value() {
        assert_eq!(MIME_TYPE, "public.mime-type");
    }

    #[test]
    fn os_type_matches_expected_value() {
        assert_eq!(OS_TYPE, "com.apple.ostype");
    }

    #[test]
    fn tag_class_values_are_distinct() {
        assert_ne!(FILENAME_EXTENSION, MIME_TYPE);
        assert_ne!(FILENAME_EXTENSION, OS_TYPE);
        assert_ne!(MIME_TYPE, OS_TYPE);
    }
}

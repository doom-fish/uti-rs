//! `OSType` / `FourCharCode` helpers for `UTType` tag dictionaries.
//!
//! `UniformTypeIdentifiers` commonly stores classic Macintosh file-type codes
//! under the raw tag-class key `com.apple.ostype`. Those tags are always four
//! bytes long and are traditionally written as printable strings like `"PNGf"`
//! or `"PDF "`.

pub use doom_fish_utils::FourCharCode;

use crate::UTIError;

/// The raw tag-class key used for `OSType` / `FourCharCode` tags.
pub const TAG_CLASS: &str = crate::tag_class::OS_TYPE;

/// Encode a four-character `OSType` / `FourCharCode` string.
///
/// # Errors
///
/// Returns [`UTIError::InvalidArgument`] unless `code` is exactly four bytes.
pub fn encode(code: &str) -> Result<FourCharCode, UTIError> {
    FourCharCode::from_slice(code.as_bytes()).ok_or_else(|| {
        UTIError::InvalidArgument(format!("OSType must be exactly four bytes, got {code:?}"))
    })
}

/// Decode an `OSType` / `FourCharCode` into its four-character string form.
#[must_use]
pub fn decode(value: FourCharCode) -> String {
    value.as_bytes().into_iter().map(char::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UTIError;

    #[test]
    fn encode_matches_the_shared_four_char_code() {
        let encoded = encode("PNGf").unwrap();
        assert_eq!(encoded, FourCharCode::from_bytes(*b"PNGf"));
        assert_eq!(encoded.as_u32(), 0x504E_4766);
        assert_eq!(encoded.as_bytes(), *b"PNGf");
    }

    #[test]
    fn encode_round_trips_through_decode() {
        let code = "PDF ";
        let encoded = encode(code).unwrap();
        assert_eq!(decode(encoded), code);
    }

    #[test]
    fn encode_rejects_non_four_byte_strings() {
        let err = encode("png").unwrap_err();
        assert_eq!(
            err,
            UTIError::InvalidArgument("OSType must be exactly four bytes, got \"png\"".to_owned())
        );
    }

    #[test]
    fn decode_preserves_padding_spaces() {
        assert_eq!(decode(FourCharCode::from_bytes(*b"PDF ")), "PDF ");
    }

    #[test]
    fn tag_class_alias_matches_public_constant() {
        assert_eq!(TAG_CLASS, crate::tag_class::OS_TYPE);
    }
}

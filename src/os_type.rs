//! `OSType` / `FourCharCode` helpers for `UTType` tag dictionaries.
//!
//! `UniformTypeIdentifiers` commonly stores classic Macintosh file-type codes
//! under the raw tag-class key `com.apple.ostype`. Those tags are always four
//! bytes long and are traditionally written as printable strings like `"PNGf"`
//! or `"PDF "`.

use crate::UTIError;

/// The raw tag-class key used for `OSType` / `FourCharCode` tags.
pub const TAG_CLASS: &str = crate::tag_class::OS_TYPE;

/// Encode a 4-byte `OSType` / `FourCharCode` into its big-endian integer form.
#[must_use]
pub const fn encode_bytes(bytes: [u8; 4]) -> u32 {
    u32::from_be_bytes(bytes)
}

/// Encode a four-character `OSType` / `FourCharCode` string.
///
/// # Errors
///
/// Returns [`UTIError::InvalidArgument`] unless `code` is exactly four bytes.
pub fn encode(code: &str) -> Result<u32, UTIError> {
    let bytes = code.as_bytes();
    let code_bytes: [u8; 4] = bytes.try_into().map_err(|_| {
        UTIError::InvalidArgument(format!("OSType must be exactly four bytes, got {code:?}"))
    })?;
    Ok(encode_bytes(code_bytes))
}

/// Decode an `OSType` / `FourCharCode` into its raw bytes.
#[must_use]
pub const fn decode_bytes(value: u32) -> [u8; 4] {
    value.to_be_bytes()
}

/// Decode an `OSType` / `FourCharCode` into its four-character string form.
#[must_use]
pub fn decode(value: u32) -> String {
    decode_bytes(value).into_iter().map(char::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UTIError;

    #[test]
    fn encode_bytes_round_trips_through_decode_bytes() {
        let bytes = *b"PNGf";
        let encoded = encode_bytes(bytes);
        assert_eq!(decode_bytes(encoded), bytes);
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
        let value = encode_bytes(*b"PDF ");
        assert_eq!(decode(value), "PDF ");
    }

    #[test]
    fn tag_class_alias_matches_public_constant() {
        assert_eq!(TAG_CLASS, crate::tag_class::OS_TYPE);
    }
}

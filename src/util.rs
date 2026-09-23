use core::ffi::c_char;
use std::collections::BTreeMap;
use std::ffi::{CStr, CString, OsString};
use std::os::unix::ffi::OsStringExt;
use std::path::{Path, PathBuf};

use crate::ffi;
use crate::UTIError;

pub fn c_string(s: &str) -> Result<CString, UTIError> {
    CString::new(s).map_err(|e| UTIError::InvalidArgument(e.to_string()))
}

pub fn path_c_string(path: &Path) -> Result<CString, UTIError> {
    let path = path.to_str().ok_or_else(|| {
        UTIError::InvalidArgument(format!("path is not valid UTF-8: {}", path.display()))
    })?;
    c_string(path)
}

/// Converts a C string pointer to a Rust `String` and frees the underlying allocation.
///
/// # Safety
///
/// The caller must ensure:
/// - `p` is either null or points to valid, properly null-terminated memory allocated by the C API.
/// - The pointer is only passed once (ownership is transferred; the C allocation is freed).
/// - If non-null, the memory layout matches the C API's allocation (as returned by FFI functions).
pub unsafe fn take_string(p: *mut c_char) -> Option<String> {
    doom_fish_utils::ffi_string::take_owned_cstring_c(p, |p| ffi::uti_string_free(p))
}

pub unsafe fn take_path(p: *mut c_char) -> Option<PathBuf> {
    if p.is_null() {
        return None;
    }
    let bytes = CStr::from_ptr(p).to_bytes().to_vec();
    ffi::uti_string_free(p);
    Some(PathBuf::from(OsString::from_vec(bytes)))
}

fn invalid_bridge_json(error: &serde_json::Error) -> UTIError {
    UTIError::OperationFailed(format!("invalid bridge list: {error}"))
}

/// Converts a C string pointer holding a JSON array of strings to a `Vec<String>` and frees the
/// allocation.
///
/// # Safety
///
/// The caller must ensure:
/// - `p` is either null or points to valid, properly null-terminated memory allocated by the C API.
/// - The pointer is only passed once (ownership is transferred; the C allocation is freed).
/// - The format is a JSON array of strings as returned by the C API.
pub unsafe fn take_string_list(p: *mut c_char) -> Result<Vec<String>, UTIError> {
    take_string(p).map_or_else(
        || Ok(Vec::new()),
        |json| serde_json::from_str(&json).map_err(|error| invalid_bridge_json(&error)),
    )
}

/// Converts a C string pointer holding a JSON object of string arrays to a
/// `BTreeMap<String, Vec<String>>` and frees the allocation.
///
/// # Safety
///
/// The caller must ensure:
/// - `p` is either null or points to valid, properly null-terminated memory allocated by the C API.
/// - The pointer is only passed once (ownership is transferred; the C allocation is freed).
/// - The format is a JSON object mapping strings to string arrays as returned by the C API.
pub unsafe fn take_string_multimap(
    p: *mut c_char,
) -> Result<BTreeMap<String, Vec<String>>, UTIError> {
    take_string(p).map_or_else(
        || Ok(BTreeMap::new()),
        |json| serde_json::from_str(&json).map_err(|error| invalid_bridge_json(&error)),
    )
}

/// Converts a C byte array pointer to a `Vec<u8>` and frees the underlying allocation.
///
/// # Safety
///
/// The caller must ensure:
/// - `p` is either null or points to valid memory allocated by the C API, with at least `len` bytes.
/// - The pointer is only passed once (ownership is transferred; the C allocation is freed).
/// - `len` accurately reflects the byte count, matching the C API's allocation.
pub unsafe fn take_bytes(p: *mut u8, len: usize) -> Vec<u8> {
    if p.is_null() {
        return Vec::new();
    }
    let bytes = if len == 0 {
        Vec::new()
    } else {
        std::slice::from_raw_parts(p, len).to_vec()
    };
    ffi::uti_bytes_free(p, len);
    bytes
}

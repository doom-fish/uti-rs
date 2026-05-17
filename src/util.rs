use core::ffi::c_char;
use std::collections::BTreeMap;
use std::ffi::CString;

use crate::ffi;
use crate::UTIError;

pub fn c_string(s: &str) -> Result<CString, UTIError> {
    CString::new(s).map_err(|e| UTIError::InvalidArgument(e.to_string()))
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
    if p.is_null() {
        return None;
    }
    let s = core::ffi::CStr::from_ptr(p).to_string_lossy().into_owned();
    ffi::uti_string_free(p);
    Some(s)
}

/// Converts a C string pointer containing newline-separated values to a `Vec<String>` and frees the allocation.
///
/// # Safety
///
/// The caller must ensure:
/// - `p` is either null or points to valid, properly null-terminated memory allocated by the C API.
/// - The pointer is only passed once (ownership is transferred; the C allocation is freed).
/// - The format is a newline-separated list as returned by the C API.
pub unsafe fn take_string_list(p: *mut c_char) -> Vec<String> {
    take_string(p).map_or_else(Vec::new, |s| {
        if s.is_empty() {
            Vec::new()
        } else {
            s.split('\n').map(str::to_owned).collect()
        }
    })
}

/// Converts a C string pointer to a `BTreeMap<String, Vec<String>>` and frees the allocation.
///
/// The input format is newline-separated lines, where each line contains tab-separated values
/// (first value is the key, remaining values are the list).
///
/// # Safety
///
/// The caller must ensure:
/// - `p` is either null or points to valid, properly null-terminated memory allocated by the C API.
/// - The pointer is only passed once (ownership is transferred; the C allocation is freed).
/// - The format is a tab/newline-delimited map as returned by the C API.
pub unsafe fn take_string_multimap(p: *mut c_char) -> BTreeMap<String, Vec<String>> {
    let mut out = BTreeMap::new();
    for line in take_string_list(p) {
        if line.is_empty() {
            continue;
        }
        let mut parts = line.split('\t');
        let Some(key) = parts.next() else {
            continue;
        };
        out.insert(key.to_string(), parts.map(str::to_owned).collect());
    }
    out
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

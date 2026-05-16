use core::ffi::c_char;
use std::collections::BTreeMap;
use std::ffi::CString;

use crate::ffi;
use crate::UTIError;

pub fn c_string(s: &str) -> Result<CString, UTIError> {
    CString::new(s).map_err(|e| UTIError::InvalidArgument(e.to_string()))
}

pub unsafe fn take_string(p: *mut c_char) -> Option<String> {
    if p.is_null() {
        return None;
    }
    let s = core::ffi::CStr::from_ptr(p).to_string_lossy().into_owned();
    ffi::uti_string_free(p);
    Some(s)
}

pub unsafe fn take_string_list(p: *mut c_char) -> Vec<String> {
    take_string(p).map_or_else(Vec::new, |s| {
        if s.is_empty() {
            Vec::new()
        } else {
            s.split('\n').map(str::to_owned).collect()
        }
    })
}

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

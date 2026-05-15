//! Raw FFI declarations matching the Swift bridge.

#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

extern "C" {
    pub fn uti_string_free(s: *mut c_char);
    pub fn uti_release(ptr: *mut c_void);
    pub fn uti_retain(ptr: *mut c_void) -> *mut c_void;

    pub fn uti_from_identifier(s: *const c_char) -> *mut c_void;
    pub fn uti_from_filename_extension(s: *const c_char) -> *mut c_void;
    pub fn uti_from_mime_type(s: *const c_char) -> *mut c_void;
    pub fn uti_from_filename_extension_conforming_to(
        ext: *const c_char,
        supertype: *mut c_void,
    ) -> *mut c_void;

    pub fn uti_identifier(ptr: *mut c_void) -> *mut c_char;
    pub fn uti_preferred_filename_extension(ptr: *mut c_void) -> *mut c_char;
    pub fn uti_preferred_mime_type(ptr: *mut c_void) -> *mut c_char;
    pub fn uti_localized_description(ptr: *mut c_void) -> *mut c_char;

    pub fn uti_is_dynamic(ptr: *mut c_void) -> bool;
    pub fn uti_is_declared(ptr: *mut c_void) -> bool;
    pub fn uti_is_public_type(ptr: *mut c_void) -> bool;

    pub fn uti_conforms_to(ptr: *mut c_void, other: *mut c_void) -> bool;
    pub fn uti_equals(ptr: *mut c_void, other: *mut c_void) -> bool;

    pub fn uti_well_known(name: *const c_char) -> *mut c_void;
}

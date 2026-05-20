//! Raw FFI declarations matching the Swift bridge.

#![allow(missing_docs)]

use core::ffi::{c_char, c_void};

#[cfg(feature = "async")]
pub type ItemProviderDataAsyncCallback =
    unsafe extern "C" fn(bytes: *const u8, len: usize, error: *const c_char, ctx: *mut c_void);

#[cfg(feature = "async")]
pub type ItemProviderFileAsyncCallback = unsafe extern "C" fn(
    path: *const c_char,
    open_in_place: bool,
    error: *const c_char,
    ctx: *mut c_void,
);

extern "C" {
    pub fn uti_string_free(s: *mut c_char);
    pub fn uti_bytes_free(bytes: *mut u8, len: usize);
    pub fn uti_release(ptr: *mut c_void);
    pub fn uti_retain(ptr: *mut c_void) -> *mut c_void;

    pub fn uti_from_identifier(s: *const c_char) -> *mut c_void;
    pub fn uti_from_filename_extension(s: *const c_char) -> *mut c_void;
    pub fn uti_from_filename_extension_conforming_to(
        ext: *const c_char,
        supertype: *mut c_void,
    ) -> *mut c_void;
    pub fn uti_from_mime_type(s: *const c_char) -> *mut c_void;
    pub fn uti_from_mime_type_conforming_to(
        mime: *const c_char,
        supertype: *mut c_void,
    ) -> *mut c_void;
    pub fn uti_from_tag(
        tag: *const c_char,
        tag_class: *const c_char,
        supertype: *mut c_void,
    ) -> *mut c_void;
    pub fn uti_types_with_tag(
        tag: *const c_char,
        tag_class: *const c_char,
        supertype: *mut c_void,
    ) -> *mut c_char;
    pub fn uti_exported_type_with_identifier(identifier: *const c_char) -> *mut c_void;
    pub fn uti_exported_type_with_identifier_conforming_to(
        identifier: *const c_char,
        parent_type: *mut c_void,
    ) -> *mut c_void;
    pub fn uti_imported_type_with_identifier(identifier: *const c_char) -> *mut c_void;
    pub fn uti_imported_type_with_identifier_conforming_to(
        identifier: *const c_char,
        parent_type: *mut c_void,
    ) -> *mut c_void;

    pub fn uti_identifier(ptr: *mut c_void) -> *mut c_char;
    pub fn uti_preferred_filename_extension(ptr: *mut c_void) -> *mut c_char;
    pub fn uti_preferred_mime_type(ptr: *mut c_void) -> *mut c_char;
    pub fn uti_localized_description(ptr: *mut c_void) -> *mut c_char;
    pub fn uti_version(ptr: *mut c_void, out_value: *mut f64) -> bool;
    pub fn uti_version_number(ptr: *mut c_void, out_value: *mut i64) -> bool;
    pub fn uti_reference_url(ptr: *mut c_void) -> *mut c_char;
    pub fn uti_tags(ptr: *mut c_void) -> *mut c_char;

    pub fn uti_is_dynamic(ptr: *mut c_void) -> bool;
    pub fn uti_is_declared(ptr: *mut c_void) -> bool;
    pub fn uti_is_public_type(ptr: *mut c_void) -> bool;

    pub fn uti_conforms_to(ptr: *mut c_void, other: *mut c_void) -> bool;
    pub fn uti_is_supertype_of(ptr: *mut c_void, other: *mut c_void) -> bool;
    pub fn uti_is_subtype_of(ptr: *mut c_void, other: *mut c_void) -> bool;
    pub fn uti_supertypes(ptr: *mut c_void) -> *mut c_char;
    pub fn uti_equals(ptr: *mut c_void, other: *mut c_void) -> bool;

    pub fn uti_string_appending_path_component_conforming_to(
        base: *const c_char,
        partial: *const c_char,
        content_type: *mut c_void,
    ) -> *mut c_char;
    pub fn uti_string_appending_path_extension_for_type(
        base: *const c_char,
        content_type: *mut c_void,
    ) -> *mut c_char;
    pub fn uti_url_appending_path_component_conforming_to(
        base_url: *const c_char,
        partial: *const c_char,
        content_type: *mut c_void,
    ) -> *mut c_char;
    pub fn uti_url_appending_path_extension_for_type(
        base_url: *const c_char,
        content_type: *mut c_void,
    ) -> *mut c_char;

    pub fn uti_well_known(name: *const c_char) -> *mut c_void;

    pub fn item_provider_new() -> *mut c_void;
    pub fn item_provider_release(ptr: *mut c_void);
    pub fn item_provider_retain(ptr: *mut c_void) -> *mut c_void;
    pub fn item_provider_from_file_path(
        path: *const c_char,
        content_type: *mut c_void,
        open_in_place: bool,
        coordinated: bool,
        visibility: i64,
        error_out: *mut *mut c_char,
    ) -> *mut c_void;
    pub fn item_provider_register_data_representation(
        provider: *mut c_void,
        content_type: *mut c_void,
        visibility: i64,
        bytes: *const u8,
        len: usize,
    );
    pub fn item_provider_register_file_representation(
        provider: *mut c_void,
        content_type: *mut c_void,
        visibility: i64,
        open_in_place: bool,
        path: *const c_char,
        coordinated: bool,
    );
    pub fn item_provider_registered_type_identifiers(provider: *mut c_void) -> *mut c_char;
    pub fn item_provider_registered_type_identifiers_with_file_options(
        provider: *mut c_void,
        open_in_place: bool,
    ) -> *mut c_char;
    pub fn item_provider_load_data_representation(
        provider: *mut c_void,
        content_type: *mut c_void,
        out_len: *mut usize,
        error_out: *mut *mut c_char,
    ) -> *mut u8;
    pub fn item_provider_load_file_representation(
        provider: *mut c_void,
        content_type: *mut c_void,
        open_in_place: bool,
        out_open_in_place: *mut bool,
        error_out: *mut *mut c_char,
    ) -> *mut c_char;
}

#[cfg(feature = "async")]
extern "C" {
    pub fn item_provider_load_data_representation_async(
        provider: *mut c_void,
        content_type: *mut c_void,
        cb: ItemProviderDataAsyncCallback,
        ctx: *mut c_void,
    );
    pub fn item_provider_load_file_representation_async(
        provider: *mut c_void,
        content_type: *mut c_void,
        open_in_place: bool,
        cb: ItemProviderFileAsyncCallback,
        ctx: *mut c_void,
    );
}

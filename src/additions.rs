//! Helpers mirroring `UTAdditions.h` path and URL utilities.

use std::path::{Path, PathBuf};

use crate::ffi;
use crate::util::{c_string, take_string};
use crate::{UTIError, UTI};

fn operation_failed(name: &str) -> UTIError {
    UTIError::OperationFailed(format!("{name} failed"))
}

/// Append a generated filename component that conforms to `content_type` to
/// `base_path`.
///
/// Mirrors `NSString -stringByAppendingPathComponent:conformingToType:`.
///
/// # Errors
///
/// Returns [`UTIError::InvalidArgument`] if the strings contain interior NUL
/// bytes, or [`UTIError::OperationFailed`] if Foundation could not construct a
/// path.
pub fn append_path_component_conforming_to<P: AsRef<Path>>(
    base_path: P,
    partial_name: &str,
    content_type: &UTI,
) -> Result<PathBuf, UTIError> {
    let base_path = c_string(&base_path.as_ref().as_os_str().to_string_lossy())?;
    let partial_name = c_string(partial_name)?;
    let out = unsafe {
        ffi::uti_string_appending_path_component_conforming_to(
            base_path.as_ptr(),
            partial_name.as_ptr(),
            content_type.as_ptr(),
        )
    };
    unsafe { take_string(out) }
        .map(PathBuf::from)
        .ok_or_else(|| operation_failed("append_path_component_conforming_to"))
}

/// Append the preferred filename extension for `content_type` to `path`.
///
/// Mirrors `NSString -stringByAppendingPathExtensionForType:`.
///
/// # Errors
///
/// Returns [`UTIError::InvalidArgument`] if the path contains interior NUL
/// bytes, or [`UTIError::OperationFailed`] if Foundation could not construct a
/// path.
pub fn append_path_extension_for_type<P: AsRef<Path>>(
    path: P,
    content_type: &UTI,
) -> Result<PathBuf, UTIError> {
    let path = c_string(&path.as_ref().as_os_str().to_string_lossy())?;
    let out = unsafe {
        ffi::uti_string_appending_path_extension_for_type(path.as_ptr(), content_type.as_ptr())
    };
    unsafe { take_string(out) }
        .map(PathBuf::from)
        .ok_or_else(|| operation_failed("append_path_extension_for_type"))
}

/// Append a generated URL path component that conforms to `content_type`.
///
/// Mirrors `NSURL -URLByAppendingPathComponent:conformingToType:`.
///
/// # Errors
///
/// Returns [`UTIError::InvalidArgument`] if the URL or partial name contains
/// interior NUL bytes, or [`UTIError::OperationFailed`] if the URL could not
/// be parsed.
pub fn append_url_path_component_conforming_to(
    base_url: &str,
    partial_name: &str,
    content_type: &UTI,
) -> Result<String, UTIError> {
    let base_url = c_string(base_url)?;
    let partial_name = c_string(partial_name)?;
    let out = unsafe {
        ffi::uti_url_appending_path_component_conforming_to(
            base_url.as_ptr(),
            partial_name.as_ptr(),
            content_type.as_ptr(),
        )
    };
    unsafe { take_string(out) }
        .ok_or_else(|| operation_failed("append_url_path_component_conforming_to"))
}

/// Append the preferred filename extension for `content_type` to `base_url`.
///
/// Mirrors `NSURL -URLByAppendingPathExtensionForType:`.
///
/// # Errors
///
/// Returns [`UTIError::InvalidArgument`] if the URL contains interior NUL
/// bytes, or [`UTIError::OperationFailed`] if the URL could not be parsed.
pub fn append_url_path_extension_for_type(
    base_url: &str,
    content_type: &UTI,
) -> Result<String, UTIError> {
    let base_url = c_string(base_url)?;
    let out = unsafe {
        ffi::uti_url_appending_path_extension_for_type(base_url.as_ptr(), content_type.as_ptr())
    };
    unsafe { take_string(out) }
        .ok_or_else(|| operation_failed("append_url_path_extension_for_type"))
}

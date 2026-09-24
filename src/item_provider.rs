//! Typed `NSItemProvider` helpers backed by `UTType` content types.

use core::ffi::c_void;
use core::ptr;
use std::fmt;
use std::path::{Path, PathBuf};

use crate::ffi;
use crate::util::{path_c_string, take_bytes, take_path, take_string, take_string_list};
use crate::{UTIError, UTI};

fn identifiers_to_utis(raw: *mut i8) -> Result<Vec<UTI>, UTIError> {
    unsafe { take_string_list(raw) }?
        .into_iter()
        .map(|identifier| UTI::from_identifier(&identifier))
        .collect()
}

fn operation_failed(name: &str) -> UTIError {
    UTIError::OperationFailed(format!("{name} failed"))
}

/// Visibility for `NSItemProvider` representations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum RepresentationVisibility {
    /// All processes can access the representation.
    All = 0,
    /// Only the same app group can access the representation.
    Group = 2,
    /// Only the originating process can access the representation.
    OwnProcess = 3,
}

impl RepresentationVisibility {
    const fn as_raw(self) -> i64 {
        self as i64
    }
}

/// A file representation loaded from an item provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedFileRepresentation {
    /// Local filesystem path: the original file when opened in place,
    /// otherwise the copy in the destination directory.
    pub path: PathBuf,
    /// Whether the file was opened in place rather than copied.
    pub open_in_place: bool,
}

/// Safe wrapper around `NSItemProvider` for typed `UTI` content operations.
#[repr(transparent)]
pub struct ItemProvider {
    ptr: *mut c_void,
}

impl ItemProvider {
    fn from_raw(ptr: *mut c_void) -> Result<Self, UTIError> {
        if ptr.is_null() {
            Err(operation_failed("item provider construction"))
        } else {
            Ok(Self { ptr })
        }
    }

    #[cfg(feature = "async")]
    pub(crate) const fn as_raw(&self) -> *mut c_void {
        self.ptr
    }

    /// Create an empty item provider.
    #[must_use]
    pub fn new() -> Self {
        let ptr = unsafe { ffi::item_provider_new() };
        debug_assert!(!ptr.is_null());
        Self { ptr }
    }

    /// Create an item provider backed by a file path.
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::InvalidArgument`] if the path contains interior NUL
    /// bytes or is not valid UTF-8, or [`UTIError::OperationFailed`] if
    /// Foundation failed to create the provider.
    pub fn from_file_path<P: AsRef<Path>>(
        path: P,
        content_type: Option<&UTI>,
        open_in_place: bool,
        coordinated: bool,
        visibility: RepresentationVisibility,
    ) -> Result<Self, UTIError> {
        let path = path_c_string(path.as_ref())?;
        let mut error = ptr::null_mut();
        let ptr = unsafe {
            ffi::item_provider_from_file_path(
                path.as_ptr(),
                content_type.map_or(ptr::null_mut(), UTI::as_ptr),
                open_in_place,
                coordinated,
                visibility.as_raw(),
                &raw mut error,
            )
        };
        if let Some(message) = unsafe { take_string(error) } {
            if !ptr.is_null() {
                unsafe { ffi::item_provider_release(ptr) };
            }
            return Err(UTIError::OperationFailed(message));
        }
        Self::from_raw(ptr)
    }

    /// Register an in-memory data representation for `content_type`.
    pub fn register_data_representation(
        &self,
        content_type: &UTI,
        visibility: RepresentationVisibility,
        data: &[u8],
    ) {
        unsafe {
            ffi::item_provider_register_data_representation(
                self.ptr,
                content_type.as_ptr(),
                visibility.as_raw(),
                data.as_ptr(),
                data.len(),
            );
        }
    }

    /// Register a file-backed representation for `content_type`.
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::InvalidArgument`] if the path contains interior NUL
    /// bytes or is not valid UTF-8.
    pub fn register_file_representation<P: AsRef<Path>>(
        &self,
        content_type: &UTI,
        visibility: RepresentationVisibility,
        open_in_place: bool,
        path: P,
        coordinated: bool,
    ) -> Result<(), UTIError> {
        let path = path_c_string(path.as_ref())?;
        unsafe {
            ffi::item_provider_register_file_representation(
                self.ptr,
                content_type.as_ptr(),
                visibility.as_raw(),
                open_in_place,
                path.as_ptr(),
                coordinated,
            );
        }
        Ok(())
    }

    /// The registered content types in registration order.
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::OperationFailed`] if Foundation produced invalid type
    /// identifiers.
    pub fn registered_content_types(&self) -> Result<Vec<UTI>, UTIError> {
        identifiers_to_utis(unsafe { ffi::item_provider_registered_type_identifiers(self.ptr) })
    }

    /// Content types that can be opened in place.
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::OperationFailed`] if Foundation produced invalid type
    /// identifiers.
    pub fn registered_content_types_for_open_in_place(&self) -> Result<Vec<UTI>, UTIError> {
        identifiers_to_utis(unsafe {
            ffi::item_provider_registered_type_identifiers_with_file_options(self.ptr, true)
        })
    }

    /// Registered content types that conform to `content_type`, in fidelity
    /// order.
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::OperationFailed`] if Foundation produced invalid type
    /// identifiers.
    pub fn registered_content_types_conforming_to(
        &self,
        content_type: &UTI,
    ) -> Result<Vec<UTI>, UTIError> {
        Ok(self
            .registered_content_types()?
            .into_iter()
            .filter(|candidate| candidate.conforms_to(content_type))
            .collect())
    }

    /// Access the executor-agnostic async wrappers for typed load operations.
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    #[must_use]
    pub const fn async_api(&self) -> crate::async_api::AsyncItemProvider<'_> {
        crate::async_api::AsyncItemProvider::new(self)
    }

    /// Load a representation as bytes.
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::OperationFailed`] if the provider could not load the
    /// requested representation or the call is made on the main thread.
    pub fn load_data_representation(&self, content_type: &UTI) -> Result<Vec<u8>, UTIError> {
        let mut len = 0;
        let mut error = ptr::null_mut();
        let bytes = unsafe {
            ffi::item_provider_load_data_representation(
                self.ptr,
                content_type.as_ptr(),
                &raw mut len,
                &raw mut error,
            )
        };
        if let Some(message) = unsafe { take_string(error) } {
            return Err(UTIError::OperationFailed(message));
        }
        Ok(unsafe { take_bytes(bytes, len) })
    }

    /// Load a representation as a file path.
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::InvalidArgument`] if `destination_directory` is not
    /// valid UTF-8 or contains NUL bytes, or [`UTIError::OperationFailed`] if
    /// the provider could not load the requested representation, the copy could
    /// not be moved into `destination_directory`, or the call is made on the
    /// main thread.
    pub fn load_file_representation<P: AsRef<Path>>(
        &self,
        content_type: &UTI,
        open_in_place: bool,
        destination_directory: P,
    ) -> Result<LoadedFileRepresentation, UTIError> {
        let destination_directory = path_c_string(destination_directory.as_ref())?;
        let mut actual_open_in_place = false;
        let mut error = ptr::null_mut();
        let path = unsafe {
            ffi::item_provider_load_file_representation(
                self.ptr,
                content_type.as_ptr(),
                open_in_place,
                destination_directory.as_ptr(),
                &raw mut actual_open_in_place,
                &raw mut error,
            )
        };
        if let Some(message) = unsafe { take_string(error) } {
            return Err(UTIError::OperationFailed(message));
        }
        let Some(path) = (unsafe { take_path(path) }) else {
            return Err(operation_failed("load_file_representation"));
        };
        Ok(LoadedFileRepresentation {
            path,
            open_in_place: actual_open_in_place,
        })
    }

    /// Load a representation as bytes without blocking the current thread.
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn load_data_representation_async(
        &self,
        content_type: &UTI,
    ) -> crate::async_api::LoadDataRepresentationFuture {
        self.async_api().load_data_representation(content_type)
    }

    /// Load a representation as a file path without blocking the current thread.
    #[cfg(feature = "async")]
    #[cfg_attr(docsrs, doc(cfg(feature = "async")))]
    pub fn load_file_representation_async<P: AsRef<Path>>(
        &self,
        content_type: &UTI,
        open_in_place: bool,
        destination_directory: P,
    ) -> crate::async_api::LoadFileRepresentationFuture {
        self.async_api().load_file_representation(
            content_type,
            open_in_place,
            destination_directory,
        )
    }
}

/// Creates an empty item provider via [`ItemProvider::new`].
impl Default for ItemProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Releases the retained `NSItemProvider` when the wrapper is dropped.
impl Drop for ItemProvider {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::item_provider_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

/// Formats an `ItemProvider` without exposing Cocoa internals.
impl fmt::Debug for ItemProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ItemProvider").finish_non_exhaustive()
    }
}

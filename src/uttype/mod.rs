//! Safe wrapper around Apple's `UTType` (`UniformTypeIdentifiers` framework).

use core::ffi::{c_char, c_void};
use core::ptr;
use std::ffi::CString;
use std::fmt;

use crate::error::UTIError;
use crate::ffi;

/// Represents one Uniform Type Identifier (e.g. `"public.png"`,
/// `"public.jpeg"`, `"com.apple.quicktime-movie"`).
///
/// Owns a retained reference to the underlying `UTType` and releases it
/// on drop.
#[repr(transparent)]
pub struct UTI {
    ptr: *mut c_void,
}

unsafe impl Send for UTI {}
unsafe impl Sync for UTI {}

impl Drop for UTI {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::uti_release(self.ptr) };
            self.ptr = ptr::null_mut();
        }
    }
}

impl Clone for UTI {
    fn clone(&self) -> Self {
        let p = unsafe { ffi::uti_retain(self.ptr) };
        Self { ptr: p }
    }
}

impl PartialEq for UTI {
    fn eq(&self, other: &Self) -> bool {
        unsafe { ffi::uti_equals(self.ptr, other.ptr) }
    }
}

impl Eq for UTI {}

impl fmt::Debug for UTI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UTI").field("identifier", &self.identifier()).finish()
    }
}

impl fmt::Display for UTI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.identifier())
    }
}

impl UTI {
    fn from_raw(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr })
        }
    }

    /// Look up a `UTType` by its dotted identifier (e.g. `"public.png"`).
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::InvalidArgument`] for invalid input strings and
    /// [`UTIError::NotFound`] if no type matches.
    pub fn from_identifier(s: &str) -> Result<Self, UTIError> {
        let c = CString::new(s).map_err(|e| UTIError::InvalidArgument(e.to_string()))?;
        Self::from_raw(unsafe { ffi::uti_from_identifier(c.as_ptr()) })
            .ok_or_else(|| UTIError::NotFound(format!("identifier {s:?}")))
    }

    /// Look up a `UTType` by its filename extension (e.g. `"png"`,
    /// without the leading dot).
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::InvalidArgument`] for invalid input strings and
    /// [`UTIError::NotFound`] if no type matches.
    pub fn from_filename_extension(ext: &str) -> Result<Self, UTIError> {
        let c = CString::new(ext).map_err(|e| UTIError::InvalidArgument(e.to_string()))?;
        Self::from_raw(unsafe { ffi::uti_from_filename_extension(c.as_ptr()) })
            .ok_or_else(|| UTIError::NotFound(format!("extension {ext:?}")))
    }

    /// Same as [`Self::from_filename_extension`] but constrained to types
    /// conforming to `supertype` (e.g. only image types).
    ///
    /// # Errors
    ///
    /// See [`Self::from_filename_extension`].
    pub fn from_filename_extension_conforming_to(
        ext: &str,
        supertype: &Self,
    ) -> Result<Self, UTIError> {
        let c = CString::new(ext).map_err(|e| UTIError::InvalidArgument(e.to_string()))?;
        Self::from_raw(unsafe {
            ffi::uti_from_filename_extension_conforming_to(c.as_ptr(), supertype.ptr)
        })
        .ok_or_else(|| UTIError::NotFound(format!("extension {ext:?} conforming to {supertype}")))
    }

    /// Look up a `UTType` by MIME type (e.g. `"image/png"`).
    ///
    /// # Errors
    ///
    /// See [`Self::from_filename_extension`].
    pub fn from_mime_type(mime: &str) -> Result<Self, UTIError> {
        let c = CString::new(mime).map_err(|e| UTIError::InvalidArgument(e.to_string()))?;
        Self::from_raw(unsafe { ffi::uti_from_mime_type(c.as_ptr()) })
            .ok_or_else(|| UTIError::NotFound(format!("mime {mime:?}")))
    }

    /// Look up one of Apple's well-known well-typed `UTType` constants by
    /// Swift property name (e.g. `"png"`, `"jpeg"`, `"pdf"`, `"audio"`,
    /// `"sourceCode"`, `"swiftSource"`, `"image"`, …).
    ///
    /// Returns `None` for unknown / not-yet-bridged names. See the bridge
    /// source for the full list (~80 well-known types).
    #[must_use]
    pub fn well_known(name: &str) -> Option<Self> {
        let c = CString::new(name).ok()?;
        Self::from_raw(unsafe { ffi::uti_well_known(c.as_ptr()) })
    }

    /// The dotted identifier (e.g. `"public.png"`).
    ///
    /// # Panics
    ///
    /// Panics if Apple returns a non-UTF-8 identifier (impossible for
    /// declared types, theoretically possible for dynamic types).
    #[must_use]
    pub fn identifier(&self) -> String {
        unsafe { take_string(ffi::uti_identifier(self.ptr)) }.unwrap_or_default()
    }

    /// Apple's preferred filename extension (no leading dot), or `None`
    /// if the type has no preferred extension.
    #[must_use]
    pub fn preferred_filename_extension(&self) -> Option<String> {
        unsafe { take_string(ffi::uti_preferred_filename_extension(self.ptr)) }
    }

    /// Apple's preferred MIME type (e.g. `"image/png"`), or `None`.
    #[must_use]
    pub fn preferred_mime_type(&self) -> Option<String> {
        unsafe { take_string(ffi::uti_preferred_mime_type(self.ptr)) }
    }

    /// User-facing localized description (e.g. `"PNG image"`).
    #[must_use]
    pub fn localized_description(&self) -> Option<String> {
        unsafe { take_string(ffi::uti_localized_description(self.ptr)) }
    }

    /// True if this is a dynamically-generated identifier (e.g.
    /// `"dyn.ah62d4rv4ge81g6ek"`) — typically returned for unknown
    /// extensions / MIME types.
    #[must_use]
    pub fn is_dynamic(&self) -> bool {
        unsafe { ffi::uti_is_dynamic(self.ptr) }
    }

    /// True if a type with this identifier was declared by the system or
    /// some installed app's `Info.plist`.
    #[must_use]
    pub fn is_declared(&self) -> bool {
        unsafe { ffi::uti_is_declared(self.ptr) }
    }

    /// True if the identifier starts with `"public."` (Apple-blessed).
    #[must_use]
    pub fn is_public_type(&self) -> bool {
        unsafe { ffi::uti_is_public_type(self.ptr) }
    }

    /// Returns true if `self` conforms to `other` (i.e. is `other` or a
    /// subtype of it). Mirrors `[UTType conformsToType:]`.
    #[must_use]
    pub fn conforms_to(&self, other: &Self) -> bool {
        unsafe { ffi::uti_conforms_to(self.ptr, other.ptr) }
    }
}

unsafe fn take_string(p: *mut c_char) -> Option<String> {
    if p.is_null() {
        return None;
    }
    let s = core::ffi::CStr::from_ptr(p).to_string_lossy().into_owned();
    ffi::uti_string_free(p);
    Some(s)
}

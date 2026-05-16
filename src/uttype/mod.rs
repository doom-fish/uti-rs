//! Safe wrapper around Apple's `UTType` (`UniformTypeIdentifiers` framework).

use core::ffi::c_void;
use core::ptr;
use std::collections::BTreeMap;
use std::fmt;

use crate::error::UTIError;
use crate::ffi;
use crate::util::{c_string, take_string, take_string_list, take_string_multimap};
use crate::{os_type, tag_class};

/// Represents one Uniform Type Identifier (e.g. `"public.png"`,
/// `"public.jpeg"`, `"com.apple.quicktime-movie"`).
///
/// Owns a retained reference to the underlying `UTType` and releases it on
/// drop.
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
        let ptr = unsafe { ffi::uti_retain(self.ptr) };
        Self { ptr }
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
        f.debug_struct("UTI")
            .field("identifier", &self.identifier())
            .finish()
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

    pub(crate) const fn as_ptr(&self) -> *mut c_void {
        self.ptr
    }

    /// Look up a `UTType` by its dotted identifier (e.g. `"public.png"`).
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::InvalidArgument`] for invalid input strings and
    /// [`UTIError::NotFound`] if no type matches.
    pub fn from_identifier(identifier: &str) -> Result<Self, UTIError> {
        let identifier_c = c_string(identifier)?;
        Self::from_raw(unsafe { ffi::uti_from_identifier(identifier_c.as_ptr()) })
            .ok_or_else(|| UTIError::NotFound(format!("identifier {identifier:?}")))
    }

    /// Look up a `UTType` by its filename extension (e.g. `"png"`, without
    /// the leading dot).
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::InvalidArgument`] for invalid input strings and
    /// [`UTIError::NotFound`] if no type matches.
    pub fn from_filename_extension(ext: &str) -> Result<Self, UTIError> {
        let ext_c = c_string(ext)?;
        Self::from_raw(unsafe { ffi::uti_from_filename_extension(ext_c.as_ptr()) })
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
        let ext_c = c_string(ext)?;
        Self::from_raw(unsafe {
            ffi::uti_from_filename_extension_conforming_to(ext_c.as_ptr(), supertype.ptr)
        })
        .ok_or_else(|| UTIError::NotFound(format!("extension {ext:?} conforming to {supertype}")))
    }

    /// Look up a `UTType` by MIME type (e.g. `"image/png"`).
    ///
    /// # Errors
    ///
    /// See [`Self::from_filename_extension`].
    pub fn from_mime_type(mime: &str) -> Result<Self, UTIError> {
        let mime_c = c_string(mime)?;
        Self::from_raw(unsafe { ffi::uti_from_mime_type(mime_c.as_ptr()) })
            .ok_or_else(|| UTIError::NotFound(format!("mime {mime:?}")))
    }

    /// Look up a `UTType` by MIME type constrained to `supertype`.
    ///
    /// # Errors
    ///
    /// See [`Self::from_filename_extension`].
    pub fn from_mime_type_conforming_to(mime: &str, supertype: &Self) -> Result<Self, UTIError> {
        let mime_c = c_string(mime)?;
        Self::from_raw(unsafe {
            ffi::uti_from_mime_type_conforming_to(mime_c.as_ptr(), supertype.ptr)
        })
        .ok_or_else(|| UTIError::NotFound(format!("mime {mime:?} conforming to {supertype}")))
    }

    /// Look up a `UTType` by arbitrary tag and tag class.
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::InvalidArgument`] for invalid input strings and
    /// [`UTIError::NotFound`] if no type matches the tag.
    pub fn from_tag(
        tag: &str,
        tag_class: &str,
        supertype: Option<&Self>,
    ) -> Result<Self, UTIError> {
        let tag_c = c_string(tag)?;
        let tag_class_c = c_string(tag_class)?;
        Self::from_raw(unsafe {
            ffi::uti_from_tag(
                tag_c.as_ptr(),
                tag_class_c.as_ptr(),
                supertype.map_or(ptr::null_mut(), Self::as_ptr),
            )
        })
        .ok_or_else(|| UTIError::NotFound(format!("tag {tag:?} in class {tag_class:?}")))
    }

    /// Return all known types matching a tag and tag class.
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::InvalidArgument`] if the inputs contain interior NUL
    /// bytes.
    pub fn types_with_tag(
        tag: &str,
        tag_class: &str,
        supertype: Option<&Self>,
    ) -> Result<Vec<Self>, UTIError> {
        let tag_c = c_string(tag)?;
        let tag_class_c = c_string(tag_class)?;
        unsafe {
            take_string_list(ffi::uti_types_with_tag(
                tag_c.as_ptr(),
                tag_class_c.as_ptr(),
                supertype.map_or(ptr::null_mut(), Self::as_ptr),
            ))
        }
        .into_iter()
        .map(|identifier| Self::from_identifier(&identifier))
        .collect()
    }

    /// Return all known types matching a filename extension.
    ///
    /// # Errors
    ///
    /// See [`Self::types_with_tag`].
    pub fn types_for_filename_extension(
        ext: &str,
        supertype: Option<&Self>,
    ) -> Result<Vec<Self>, UTIError> {
        Self::types_with_tag(ext, tag_class::FILENAME_EXTENSION, supertype)
    }

    /// Return all known types matching a MIME type.
    ///
    /// # Errors
    ///
    /// See [`Self::types_with_tag`].
    pub fn types_for_mime_type(mime: &str, supertype: Option<&Self>) -> Result<Vec<Self>, UTIError> {
        Self::types_with_tag(mime, tag_class::MIME_TYPE, supertype)
    }

    /// Look up a `UTType` by classic Macintosh `OSType` / `FourCharCode`.
    ///
    /// # Errors
    ///
    /// See [`Self::from_tag`].
    pub fn from_os_type(os_type_code: u32) -> Result<Self, UTIError> {
        let os_type = os_type::decode(os_type_code);
        Self::from_tag(&os_type, tag_class::OS_TYPE, None)
    }

    /// Look up a `UTType` by `OSType` / `FourCharCode` constrained to `supertype`.
    ///
    /// # Errors
    ///
    /// See [`Self::from_os_type`].
    pub fn from_os_type_conforming_to(
        os_type_code: u32,
        supertype: &Self,
    ) -> Result<Self, UTIError> {
        let os_type = os_type::decode(os_type_code);
        Self::from_tag(&os_type, tag_class::OS_TYPE, Some(supertype))
    }

    /// Return all known types matching an `OSType` / `FourCharCode`.
    ///
    /// # Errors
    ///
    /// See [`Self::types_with_tag`].
    pub fn types_for_os_type(
        os_type_code: u32,
        supertype: Option<&Self>,
    ) -> Result<Vec<Self>, UTIError> {
        let os_type = os_type::decode(os_type_code);
        Self::types_with_tag(&os_type, tag_class::OS_TYPE, supertype)
    }

    /// Look up one of Apple's well-known core types by keyword.
    ///
    /// The accepted keywords roughly follow Swift property names such as
    /// `"png"`, `"jpeg"`, `"pdf"`, `"audio"`, `"sourceCode"`, and
    /// `"swiftSource"`.
    #[must_use]
    pub fn well_known(name: &str) -> Option<Self> {
        let name = c_string(name).ok()?;
        Self::from_raw(unsafe { ffi::uti_well_known(name.as_ptr()) })
    }

    /// Construct an active type exported by the current process.
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::InvalidArgument`] for invalid input strings and
    /// [`UTIError::OperationFailed`] if the framework could not construct the
    /// exported type.
    pub fn exported_type_with_identifier(identifier: &str) -> Result<Self, UTIError> {
        let identifier_c = c_string(identifier)?;
        Self::from_raw(unsafe { ffi::uti_exported_type_with_identifier(identifier_c.as_ptr()) })
            .ok_or_else(|| UTIError::OperationFailed(format!("exported identifier {identifier:?}")))
    }

    /// Construct an active exported type constrained to `parent_type`.
    ///
    /// # Errors
    ///
    /// See [`Self::exported_type_with_identifier`].
    pub fn exported_type_with_identifier_conforming_to(
        identifier: &str,
        parent_type: &Self,
    ) -> Result<Self, UTIError> {
        let identifier_c = c_string(identifier)?;
        Self::from_raw(unsafe {
            ffi::uti_exported_type_with_identifier_conforming_to(
                identifier_c.as_ptr(),
                parent_type.ptr,
            )
        })
        .ok_or_else(|| {
            UTIError::OperationFailed(format!(
                "exported identifier {identifier:?} conforming to {parent_type}"
            ))
        })
    }

    /// Construct an active type imported by the current process.
    ///
    /// # Errors
    ///
    /// Returns [`UTIError::InvalidArgument`] for invalid input strings and
    /// [`UTIError::OperationFailed`] if the framework could not construct the
    /// imported type.
    pub fn imported_type_with_identifier(identifier: &str) -> Result<Self, UTIError> {
        let identifier_c = c_string(identifier)?;
        Self::from_raw(unsafe { ffi::uti_imported_type_with_identifier(identifier_c.as_ptr()) })
            .ok_or_else(|| UTIError::OperationFailed(format!("imported identifier {identifier:?}")))
    }

    /// Construct an active imported type constrained to `parent_type`.
    ///
    /// # Errors
    ///
    /// See [`Self::imported_type_with_identifier`].
    pub fn imported_type_with_identifier_conforming_to(
        identifier: &str,
        parent_type: &Self,
    ) -> Result<Self, UTIError> {
        let identifier_c = c_string(identifier)?;
        Self::from_raw(unsafe {
            ffi::uti_imported_type_with_identifier_conforming_to(
                identifier_c.as_ptr(),
                parent_type.ptr,
            )
        })
        .ok_or_else(|| {
            UTIError::OperationFailed(format!(
                "imported identifier {identifier:?} conforming to {parent_type}"
            ))
        })
    }

    /// The dotted identifier (e.g. `"public.png"`).
    ///
    /// # Panics
    ///
    /// Panics if Apple returns a non-UTF-8 identifier.
    #[must_use]
    pub fn identifier(&self) -> String {
        unsafe { take_string(ffi::uti_identifier(self.ptr)) }.unwrap_or_default()
    }

    /// Apple's preferred filename extension (no leading dot), or `None` if the
    /// type has no preferred extension.
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

    /// The declared version number for the type, if present.
    ///
    /// This remains a floating-point compatibility shim for v0.3 callers; use
    /// [`Self::version_number`] for the underlying integer/`NSNumber` value.
    #[must_use]
    pub fn version(&self) -> Option<f64> {
        let mut value = 0.0;
        unsafe { ffi::uti_version(self.ptr, &mut value) }.then_some(value)
    }

    /// The declared integer version number for the type, if present.
    #[must_use]
    pub fn version_number(&self) -> Option<i64> {
        let mut value = 0_i64;
        unsafe { ffi::uti_version_number(self.ptr, &mut value) }.then_some(value)
    }

    /// A reference URL describing the type, if present.
    #[must_use]
    pub fn reference_url(&self) -> Option<String> {
        unsafe { take_string(ffi::uti_reference_url(self.ptr)) }
    }

    /// The normalized tag-specification dictionary for this type.
    #[must_use]
    pub fn tags(&self) -> BTreeMap<String, Vec<String>> {
        unsafe { take_string_multimap(ffi::uti_tags(self.ptr)) }
    }

    /// All tags for a specific tag class.
    #[must_use]
    pub fn tag_values(&self, tag_class: &str) -> Vec<String> {
        self.tags().remove(tag_class).unwrap_or_default()
    }

    /// All filename extensions declared for this type.
    #[must_use]
    pub fn filename_extensions(&self) -> Vec<String> {
        self.tag_values(tag_class::FILENAME_EXTENSION)
    }

    /// All MIME types declared for this type.
    #[must_use]
    pub fn mime_types(&self) -> Vec<String> {
        self.tag_values(tag_class::MIME_TYPE)
    }

    /// All `OSType` / `FourCharCode` tags declared for this type.
    #[must_use]
    pub fn os_type_strings(&self) -> Vec<String> {
        self.tag_values(tag_class::OS_TYPE)
    }

    /// The preferred `OSType` / `FourCharCode` string for this type, if present.
    #[must_use]
    pub fn preferred_os_type_string(&self) -> Option<String> {
        self.os_type_strings().into_iter().next()
    }

    /// All `OSType` / `FourCharCode` tags declared for this type, decoded as `u32`.
    #[must_use]
    pub fn os_types(&self) -> Vec<u32> {
        self.os_type_strings()
            .into_iter()
            .filter_map(|value| os_type::encode(&value).ok())
            .collect()
    }

    /// The preferred `OSType` / `FourCharCode` for this type, if present.
    #[must_use]
    pub fn preferred_os_type(&self) -> Option<u32> {
        self.preferred_os_type_string()
            .and_then(|value| os_type::encode(&value).ok())
    }

    /// True if this is a dynamically-generated identifier (e.g.
    /// `"dyn.ah62d4rv4ge81g6ek"`).
    #[must_use]
    pub fn is_dynamic(&self) -> bool {
        unsafe { ffi::uti_is_dynamic(self.ptr) }
    }

    /// True if a type with this identifier was declared by the system or some
    /// installed app's `Info.plist`.
    #[must_use]
    pub fn is_declared(&self) -> bool {
        unsafe { ffi::uti_is_declared(self.ptr) }
    }

    /// True if the type is in Apple's public namespace.
    #[must_use]
    pub fn is_public_type(&self) -> bool {
        unsafe { ffi::uti_is_public_type(self.ptr) }
    }

    /// True if the type is in Apple's public namespace.
    #[must_use]
    pub fn is_public(&self) -> bool {
        self.is_public_type()
    }

    /// Returns true if `self` conforms to `other` (i.e. is `other` or a subtype
    /// of it).
    #[must_use]
    pub fn conforms_to(&self, other: &Self) -> bool {
        unsafe { ffi::uti_conforms_to(self.ptr, other.ptr) }
    }

    /// Returns true if `self` is a strict supertype of `other`.
    #[must_use]
    pub fn is_supertype_of(&self, other: &Self) -> bool {
        unsafe { ffi::uti_is_supertype_of(self.ptr, other.ptr) }
    }

    /// Returns true if `self` is a strict subtype of `other`.
    #[must_use]
    pub fn is_subtype_of(&self, other: &Self) -> bool {
        unsafe { ffi::uti_is_subtype_of(self.ptr, other.ptr) }
    }

    /// All supertypes of this type, sorted by identifier for deterministic
    /// iteration.
    #[must_use]
    pub fn supertypes(&self) -> Vec<Self> {
        unsafe { take_string_list(ffi::uti_supertypes(self.ptr)) }
            .into_iter()
            .filter_map(|identifier| Self::from_identifier(&identifier).ok())
            .collect()
    }
}

/// Swift-facing alias matching `UniformTypeIdentifiers.UTType`.
pub type UTType = UTI;

/// Obj-C / apinotes alias matching Apple's reference-semantics name.
pub type UTTypeReference = UTI;

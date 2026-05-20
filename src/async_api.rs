//! Async API for `ItemProvider` (requires the `async` cargo feature).
//!
//! Provides executor-agnostic [`Future`] wrappers around `NSItemProvider`'s
//! typed completion-handler loading APIs. Works with any async runtime without
//! depending on a specific executor.
//!
//! ## Available types
//!
//! | Type | Description |
//! |------|-------------|
//! | [`AsyncItemProvider`] | Async accessors for typed `NSItemProvider` loads |
//! | [`LoadDataRepresentationFuture`] | Future for `loadDataRepresentationForContentType` |
//! | [`LoadFileRepresentationFuture`] | Future for `loadFileRepresentationForContentType` |
//!
//! ## Example
//!
//! ```rust,no_run
//! # #[cfg(feature = "async")]
//! # fn main() -> Result<(), Box<dyn std::error::Error>> { pollster::block_on(async {
//! use uti::{ItemProvider, RepresentationVisibility, UTI};
//!
//! let provider = ItemProvider::new();
//! let plain_text = UTI::well_known("plainText").unwrap();
//! provider.register_data_representation(
//!     &plain_text,
//!     RepresentationVisibility::OwnProcess,
//!     b"hello from async item provider",
//! );
//!
//! let bytes = provider.load_data_representation_async(&plain_text).await?;
//! assert_eq!(bytes, b"hello from async item provider");
//! # Ok(()) }) }
//! # #[cfg(not(feature = "async"))]
//! # fn main() {}
//! ```

#![allow(clippy::missing_errors_doc, clippy::must_use_candidate)]

use std::ffi::{c_char, c_void, CStr};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use doom_fish_utils::completion::{error_from_cstr, AsyncCompletion, AsyncCompletionFuture};

use crate::ffi;
use crate::{ItemProvider, LoadedFileRepresentation, UTIError, UTI};

const fn bridge_err(message: String) -> UTIError {
    UTIError::OperationFailed(message)
}

unsafe extern "C" fn item_provider_data_completion_cb(
    bytes: *const u8,
    len: usize,
    error: *const c_char,
    ctx: *mut c_void,
) {
    if !error.is_null() {
        let message = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<Vec<u8>>::complete_err(ctx, message) };
        return;
    }

    let data = if bytes.is_null() {
        if len == 0 {
            Vec::new()
        } else {
            unsafe {
                AsyncCompletion::<Vec<u8>>::complete_err(
                    ctx,
                    "item provider async bridge returned null bytes".into(),
                );
            };
            return;
        }
    } else {
        unsafe { std::slice::from_raw_parts(bytes, len).to_vec() }
    };

    unsafe { AsyncCompletion::complete_ok(ctx, data) };
}

unsafe extern "C" fn item_provider_file_completion_cb(
    path: *const c_char,
    open_in_place: bool,
    error: *const c_char,
    ctx: *mut c_void,
) {
    if !error.is_null() {
        let message = unsafe { error_from_cstr(error) };
        unsafe { AsyncCompletion::<LoadedFileRepresentation>::complete_err(ctx, message) };
        return;
    }

    if path.is_null() {
        unsafe {
            AsyncCompletion::<LoadedFileRepresentation>::complete_err(
                ctx,
                "loadFileRepresentation returned no file URL".into(),
            );
        };
        return;
    }

    let path = unsafe { CStr::from_ptr(path).to_string_lossy().into_owned() };
    unsafe {
        AsyncCompletion::complete_ok(
            ctx,
            LoadedFileRepresentation {
                path,
                open_in_place,
            },
        );
    };
}

/// Future returned by [`AsyncItemProvider::load_data_representation`].
#[must_use = "futures do nothing unless polled"]
pub struct LoadDataRepresentationFuture {
    inner: AsyncCompletionFuture<Vec<u8>>,
}

impl std::fmt::Debug for LoadDataRepresentationFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadDataRepresentationFuture")
            .finish_non_exhaustive()
    }
}

impl Future for LoadDataRepresentationFuture {
    type Output = Result<Vec<u8>, UTIError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|result| result.map_err(bridge_err))
    }
}

/// Future returned by [`AsyncItemProvider::load_file_representation`].
#[must_use = "futures do nothing unless polled"]
pub struct LoadFileRepresentationFuture {
    inner: AsyncCompletionFuture<LoadedFileRepresentation>,
}

impl std::fmt::Debug for LoadFileRepresentationFuture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LoadFileRepresentationFuture")
            .finish_non_exhaustive()
    }
}

impl Future for LoadFileRepresentationFuture {
    type Output = Result<LoadedFileRepresentation, UTIError>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner)
            .poll(cx)
            .map(|result| result.map_err(bridge_err))
    }
}

/// Async accessors for typed `NSItemProvider` load operations.
#[derive(Clone, Copy, Debug)]
pub struct AsyncItemProvider<'a> {
    provider: &'a ItemProvider,
}

impl<'a> AsyncItemProvider<'a> {
    /// Wrap a borrowed [`ItemProvider`].
    #[must_use]
    pub const fn new(provider: &'a ItemProvider) -> Self {
        Self { provider }
    }

    /// Load a representation as bytes without blocking the current thread.
    pub fn load_data_representation(&self, content_type: &UTI) -> LoadDataRepresentationFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::item_provider_load_data_representation_async(
                self.provider.as_raw(),
                content_type.as_ptr(),
                item_provider_data_completion_cb,
                ctx,
            );
        }
        LoadDataRepresentationFuture { inner: future }
    }

    /// Load a representation as a file path without blocking the current thread.
    pub fn load_file_representation(
        &self,
        content_type: &UTI,
        open_in_place: bool,
    ) -> LoadFileRepresentationFuture {
        let (future, ctx) = AsyncCompletion::create();
        unsafe {
            ffi::item_provider_load_file_representation_async(
                self.provider.as_raw(),
                content_type.as_ptr(),
                open_in_place,
                item_provider_file_completion_cb,
                ctx,
            );
        }
        LoadFileRepresentationFuture { inner: future }
    }
}

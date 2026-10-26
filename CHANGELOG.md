# Changelog

All notable changes to `uti` are documented here.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.1] - 2026-10-26

### Fixed

- Build the Swift bridge with the default SwiftPM build engine and use its reported product directory.

## [0.6.0] - 2026-09-24

### Fixed

- `load_file_representation` and `load_file_representation_async` returned the path of the provider's temporary copy, which `NSItemProvider` deletes as soon as the completion handler returns. The copy is now moved into a caller-supplied directory inside the handler.
- Type-identifier lists and tag dictionaries crossed the bridge as newline- and tab-joined strings, so a tag containing a newline or tab was split into bogus entries. They are now JSON-encoded.
- Tag lists kept the framework's order instead of being sorted, so `preferred_os_type` and the first entry of `filename_extensions` / `mime_types` are the preferred values again.
- The synchronous loaders now return an error on the main thread instead of blocking it for up to 30 seconds, and a timed-out load is cancelled and can no longer write its result afterwards.
- Paths no longer go through `to_string_lossy`: they cross the bridge byte for byte, and paths that aren't valid UTF-8 are rejected with `UTIError::InvalidArgument` instead of silently naming a different file.
- `register_data_representation` no longer force-unwraps the byte pointer in Swift.
- The `version` / `version_number` unit tests exercised a local stub; they now call the framework.
- `core_types::APPLE_SCRIPT` was an alias for the OSA script type, `core_types::CALENDAR_EVENT` was the vCalendar file type `com.apple.ical.vcs`, and `core_types::INTERNET_SHORTCUT` and `UTI::well_known("internetShortcut")` returned the internet-location type, which is what Apple's header documents for `UTTypeInternetShortcut` but not the framework's value. `APPLE_SCRIPT` and `CALENDAR_EVENT` now hold the framework's values, and `well_known("internetShortcut")` reads its identifier from the SDK. A new test checks every `well_known` name against its `core_types` constant at runtime.
- `RepresentationVisibility::Team` passed a visibility that is unavailable on macOS, and the bridge mapped unrecognised visibility values to `.all`, the most permissive one. `Team` is gone and unrecognised values map to `.ownProcess`.

### Changed

- **Breaking:** `ItemProvider::load_file_representation`, `ItemProvider::load_file_representation_async` and `AsyncItemProvider::load_file_representation` take a destination directory.
- **Breaking:** `LoadedFileRepresentation::path` is a `PathBuf` (was `String`).
- **Breaking:** the `OSType` API uses `doom_fish_utils::FourCharCode` instead of `u32`: `os_type::encode` returns it, `os_type::decode`, `UTI::from_os_type`, `UTI::from_os_type_conforming_to` and `UTI::types_for_os_type` take it, and `UTI::os_types` / `UTI::preferred_os_type` return it.
- **Breaking:** the raw `ffi::item_provider_load_file_representation` and `ffi::item_provider_load_file_representation_async` declarations take the destination directory.
- **Breaking:** `core_types::APPLE_SCRIPT` is now `UTTypeAppleScript`'s identifier `com.apple.applescript.text` (use `OSA_SCRIPT` for `com.apple.applescript.script`), and `core_types::CALENDAR_EVENT` is now `public.calendar-event`.
- **Breaking:** `ItemProvider` no longer implements `Clone`. Cloning retained the same `NSItemProvider`, so registering a representation on one clone changed the other.
- The doom-fish-utils requirement is now `>=0.4.1, <0.5`; the old `>=0.1` lower bound allowed versions without `take_owned_cstring_c`. `rust-version` is 1.82.

### Added

- `uti::FourCharCode` and `uti::os_type::FourCharCode` re-exports.
- Tests for kept file copies, in-place and Unicode path round trips, non-UTF-8 path rejection, tags containing separators, and main-thread refusal (a `harness = false` test that runs on the process main thread).

### Removed

- **Breaking:** removed the `core_types::BMP`, `ICO`, `WAV` and `EXE` string constants. Use `UTI::well_known("bmp")`, `"ico"`, `"wav"` or `"exe"`; the bridge now reads those identifiers from the SDK's `UTType` constants.
- **Breaking:** `os_type::encode_bytes` and `os_type::decode_bytes`; use `FourCharCode::from_bytes` and `FourCharCode::as_bytes`.
- **Breaking:** the floating-point `UTI::version()` compatibility shim and the raw `ffi::uti_version`; use `UTI::version_number()`.
- **Breaking:** `RepresentationVisibility::Team`, which macOS doesn't support.
- **Breaking:** the legacy constants `core_types::APPLE_SCRIPT_TEXT`, `CALENDAR_EVENT_ITEM` and `ARKIT_REALITY_FILE` (use `APPLE_SCRIPT`, `CALENDAR_EVENT` and `REALITY_FILE`), `core_types::INTERNET_SHORTCUT` (use `UTI::well_known("internetShortcut")`), and the duplicate `well_known` names `"alias"`, `"appleScriptText"`, `"arkitRealityFile"` and `"calendarEventItem"`.
- **Breaking:** the raw `ffi::item_provider_retain`.

## [0.5.2] - 2026-06-06

- `ItemProvider::from_file_path` no longer leaks the provider when Foundation returns both an error and a provider, and `UTI::identifier()` lost its incorrect `# Panics` note.

## [0.5.1] - 2026-05-20

- Migrated local `take_string` body to call `doom_fish_utils::ffi_string::take_owned_cstring_c`. Centralises the duplicated FFI take-string pattern fleet-wide. No public API change.

## [0.5.0] - 2026-05-20

### Added

- Optional `async` feature with executor-agnostic `async_api::AsyncItemProvider` plus `ItemProvider::{load_data_representation_async, load_file_representation_async}` for `NSItemProvider`'s typed completion-handler loaders.
- New async example `examples/07_item_provider_async.rs` and integration coverage in `tests/item_provider_async.rs`.

### Changed

- Extended the Swift bridge and Rust FFI with non-blocking typed `NSItemProvider` load thunks while preserving the existing synchronous API.
- Refreshed README and coverage docs against `MacOSX26.5.sdk`.

### Notes

- Phase 32 completeness + async sweep.

## [v0.4.4]

### Added

- In-`src/` unit tests across `os_type`, `tag_class`, `error`, and `uttype`, providing fast `cargo test --lib` fail-fast signal alongside the existing integration tests under `tests/`.

## [v0.4.3]

### Changed

- Added concise rustdoc coverage to every public item in non-generated `src/` code, including `core_types::*`, `UTIError`, and the release-facing trait impls.
- Raised nightly `cargo +nightly rustdoc --lib --all-features -- -Z unstable-options --show-coverage` documentation coverage for non-generated `src/` files to 100.0%.

## [v0.4.2]

### Fixed

- Added comprehensive SAFETY documentation to all unsafe functions in `util.rs` (`take_string`, `take_string_list`, `take_string_multimap`, `take_bytes`), clarifying preconditions and pointer ownership semantics.
- Added SAFETY documentation to `unsafe impl Send` and `unsafe impl Sync` for `UTI`, explaining thread-safety guarantees of the underlying Foundation `UTType`.

## [v0.4.1]

### Added

- Five new integration test files under `tests/` covering UTType creation/lookup, conformance, filename-extension mapping, MIME-type mapping, and OSType / FourCharCode encoding.

## [v0.4.0]

### Added

- `UTType` / `UTTypeReference` aliases for the existing `UTI` wrapper, matching Apple's Swift/apinotes naming.
- Integer-safe `UTI::version_number()` plus `UTI::is_public()` alias for the framework's `isPublic` property.
- OSType / FourCharCode conveniences: `tag_class::OS_TYPE`, `os_type::{encode, decode, encode_bytes, decode_bytes}`, `UTI::from_os_type(...)`, `types_for_os_type(...)`, `preferred_os_type()`, and `os_types()`.
- Explicit filename-extension / MIME multi-match helpers: `types_for_filename_extension(...)` and `types_for_mime_type(...)`.
- New smoke test file `tests/uttype_helpers.rs`, new example `examples/06_dynamic_ostype.rs`, and a generated `COVERAGE.md` audit ledger.

### Changed

- Split the Swift bridge into logical area files (`UTI.swift`, `Additions.swift`, `ItemProvider.swift`) following the gold-standard multi-file pattern.
- `NSItemProvider` bridge calls now use the typed `UTType` overloads directly.
- Public-type checks now call the SDK's `isPublic` property instead of inferring from the identifier prefix.

## [v0.3.0]

### Added

- Full `UTType.h` coverage: MIME-constrained constructors, generic tag lookup,
  `types_with_tag`, `version`, `reference_url`, tag dictionaries, strict
  subtype / supertype checks, `supertypes`, and local exported / imported
  types.
- `tag_class` module exposing `FILENAME_EXTENSION` and `MIME_TYPE`.
- `additions` module covering the `UTAdditions.h` path / URL helpers.
- `ItemProvider`, `RepresentationVisibility`, and `LoadedFileRepresentation`
  for typed `NSItemProvider` workflows.
- Full `UTCoreTypes.h` coverage in `core_types::*` and `UTI::well_known(...)`.
- Three new smoke examples: `03_tags`, `04_additions`, and `05_item_provider`.
- Expanded API-coverage tests for `UTType`, `UTCoreTypes`, `UTAdditions`, and
  typed `NSItemProvider` glue.

### Changed

- Preserved legacy aliases like `APPLE_SCRIPT`, `CALENDAR_EVENT`,
  `ARKIT_REALITY_FILE`, and `KERNEL_EXTENSION` while adding the current SDK
  constants alongside them.
- Publish tarballs now include examples and tests.

## [0.2.0] - 2026-05-16

### Added

- Add named UTI constants in the `core_types` module.

## [0.1.0] - Initial release

### Added

- `UTI` — opaque wrapper around Apple's `UTType` (boxed in an NSObject so
  the value-type Swift struct can cross the FFI as a refcounted pointer).
- Constructors:
  - `UTI::from_identifier("public.png")`
  - `UTI::from_filename_extension("png")`
  - `UTI::from_filename_extension_conforming_to(ext, &supertype)`
  - `UTI::from_mime_type("image/png")`
  - `UTI::well_known("png")` — picks one of ~80 bridged Apple constants
    (`png`, `jpeg`, `pdf`, `audio`, `image`, `text`, `data`, `content`,
    `swiftSource`, `mp3`, …).
- Accessors: `identifier()`, `preferred_filename_extension()`,
  `preferred_mime_type()`, `localized_description()`.
- State queries: `is_dynamic()`, `is_declared()`, `is_public_type()`.
- Conformance: `conforms_to(&other)` + `==` equality.
- 2 examples (`01_lookup`, `02_conformance`).
- 2 API-coverage tests (`UTType` + `UTType (Conformance)` category) using
  the family's Obj-C @interface header-parsing harness.

Both `Send` + `Sync`. Ref-counted via `Clone` (cheap; just bumps the
NSObject refcount). Drops the underlying `UTType` automatically.

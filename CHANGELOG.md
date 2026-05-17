# Changelog

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

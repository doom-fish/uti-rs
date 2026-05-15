# Changelog

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

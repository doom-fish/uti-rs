# uti

Safe Rust bindings for Apple's [UniformTypeIdentifiers](https://developer.apple.com/documentation/uniformtypeidentifiers) framework on macOS — file-type and MIME identification via `UTType`.

> **Status:** v0.3 covers the current `UniformTypeIdentifiers` header surface on macOS: `UTType` constructors/accessors/conformance/tag-spec/local constants, full `UTCoreTypes.h`, `UTTagClass`, `UTAdditions`, and typed `NSItemProvider` helpers.

## Quick start

```rust,no_run
use uti::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Look up by extension, MIME, or full identifier.
    let png  = UTI::from_filename_extension("png")?;
    let json = UTI::from_mime_type("application/json")?;
    let pdf  = UTI::from_identifier("com.adobe.pdf")?;

    println!("png  identifier: {}", png.identifier());
    println!("png  mime:       {:?}", png.preferred_mime_type());
    println!("png  description:{:?}", png.localized_description());
    println!("json extension:  {:?}", json.preferred_filename_extension());
    println!("pdf  is_public:  {}", pdf.is_public_type());

    // Conformance: PNG conforms to image, image conforms to data.
    let image = UTI::well_known("image").unwrap();
    let data  = UTI::well_known("data").unwrap();
    assert!(png.conforms_to(&image));
    assert!(png.conforms_to(&data));
    Ok(())
}
```

## Pipeline composition

```text
imageio (open file) ──► uti (identify format) ──► dispatch to right pipeline
                                                    │
                                                    ├─► PNG/JPEG -> apple-vision OCR
                                                    ├─► WAV/MP3  -> soundanalysis
                                                    ├─► PDF      -> pdfkit (planned)
                                                    └─► TXT      -> naturallanguage
```

`UTI` is foundational — every doom-fish crate that takes a file path can use it to dispatch on format before invoking the right pipeline.

## Roadmap

- [x] `UTI::from_identifier(...)`, `from_filename_extension(...)`, `from_mime_type(...)`, and generic `from_tag(...)`
- [x] Accessors: `identifier`, `preferred_filename_extension`, `preferred_mime_type`, `localized_description`, `version`, `reference_url`, `tags`
- [x] Conformance: `conforms_to`, `is_supertype_of`, `is_subtype_of`, `supertypes`, equality
- [x] State queries: `is_dynamic`, `is_declared`, `is_public_type`
- [x] Full `UTCoreTypes.h` coverage via `core_types::*` + `UTI::well_known(name)`
- [x] `UTTagClass` constants via `tag_class::FILENAME_EXTENSION` and `tag_class::MIME_TYPE`
- [x] `UTAdditions` helpers via `uti::additions`
- [x] `NSItemProvider` integration via `ItemProvider`
- [x] SDK coverage tests for `UTType`, `UTCoreTypes`, `UTAdditions`, and typed `NSItemProvider`

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.

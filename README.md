# uti

Safe Rust bindings for Apple's [UniformTypeIdentifiers](https://developer.apple.com/documentation/uniformtypeidentifiers) framework on macOS — file-type and MIME identification via `UTType`.

> **Status:** v0.5.0 keeps the MacOSX26.5.sdk audit clean and adds true async `NSItemProvider` typed loaders behind the optional `async` feature. See [`COVERAGE.md`](COVERAGE.md).

## Quick start

```rust,no_run
use uti::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Look up by extension, MIME, or full identifier.
    let png  = UTI::from_filename_extension("png")?;
    let json = UTI::from_mime_type("application/json")?;
    let pdf  = UTI::from_identifier("com.adobe.pdf")?;
    let png_code = os_type::encode("PNGf")?;

    println!("png  identifier: {}", png.identifier());
    println!("png  mime:       {:?}", png.preferred_mime_type());
    println!("png  description:{:?}", png.localized_description());
    println!("json extension:  {:?}", json.preferred_filename_extension());
    println!("pdf  is_public:  {}", pdf.is_public());
    println!("png  ostype:     {:?}", png.preferred_os_type_string());
    assert_eq!(UTI::from_os_type(png_code)?.identifier(), png.identifier());

    // Conformance: PNG conforms to image, image conforms to data.
    let image = UTI::well_known("image").unwrap();
    let data  = UTI::well_known("data").unwrap();
    assert!(png.conforms_to(&image));
    assert!(png.conforms_to(&data));
    Ok(())
}
```

## Async `ItemProvider`

Enable `features = ["async"]` for runtime-neutral wrappers around `NSItemProvider`'s typed completion-handler loading APIs.

```rust,no_run
# #[cfg(feature = "async")]
# fn main() -> Result<(), Box<dyn std::error::Error>> { pollster::block_on(async {
use uti::{async_api::AsyncItemProvider, ItemProvider, RepresentationVisibility, UTI};

let provider = ItemProvider::new();
let plain_text = UTI::well_known("plainText").unwrap();
provider.register_data_representation(
    &plain_text,
    RepresentationVisibility::OwnProcess,
    b"hello from async item provider",
);

let bytes = AsyncItemProvider::new(&provider)
    .load_data_representation(&plain_text)
    .await?;
assert_eq!(bytes, b"hello from async item provider");
# Ok(()) }) }
# #[cfg(not(feature = "async"))]
# fn main() {}
```

See `examples/07_item_provider_async.rs` for a full data + file example.

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

- [x] `UTI::from_identifier(...)`, `from_filename_extension(...)`, `from_mime_type(...)`, generic `from_tag(...)`, and multi-match helpers for filename extensions / MIME types / `OSType` tags
- [x] Accessors: `identifier`, `preferred_filename_extension`, `preferred_mime_type`, `localized_description`, `version`, `version_number`, `reference_url`, `tags`, `filename_extensions`, `mime_types`, `preferred_os_type`, `os_types`
- [x] Conformance: `conforms_to`, `is_supertype_of`, `is_subtype_of`, `supertypes`, equality
- [x] State queries: `is_dynamic`, `is_declared`, `is_public_type`, `is_public`
- [x] Swift / Obj-C naming aliases: `UTType`, `UTTypeReference`
- [x] Full `UTCoreTypes.h` coverage via `core_types::*` + `UTI::well_known(name)`
- [x] `UTTagClass` constants via `tag_class::FILENAME_EXTENSION`, `tag_class::MIME_TYPE`, plus crate convenience `tag_class::OS_TYPE`
- [x] `OSType` / `FourCharCode` encoding helpers via `uti::os_type`
- [x] `UTAdditions` helpers via `uti::additions`
- [x] `NSItemProvider` integration via `ItemProvider`, plus non-blocking typed loaders via `async_api` / `ItemProvider::*_async` (`async` feature)
- [x] SDK coverage tests, smoke tests, and `COVERAGE.md` audit output

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at your option.

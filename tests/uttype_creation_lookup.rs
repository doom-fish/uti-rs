use uti::{core_types, tag_class, UTType, UTI};

#[test]
fn creation_and_lookup_helpers_resolve_the_same_png_type() {
    let by_identifier: UTType = UTI::from_identifier(core_types::PNG).unwrap();
    let by_well_known = UTI::well_known("png").unwrap();
    let by_extension = UTI::from_filename_extension("png").unwrap();
    let by_mime = UTI::from_mime_type("image/png").unwrap();
    let by_tag = UTI::from_tag("png", tag_class::FILENAME_EXTENSION, None).unwrap();

    for candidate in [&by_well_known, &by_extension, &by_mime, &by_tag] {
        assert_eq!(candidate.identifier(), core_types::PNG);
        assert_eq!(candidate, &by_identifier);
    }
}

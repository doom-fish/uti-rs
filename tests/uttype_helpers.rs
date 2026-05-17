use std::collections::BTreeSet;

use uti::{core_types, os_type, UTType, UTTypeReference, UTI};

#[test]
fn uttype_aliases_preserve_identity() {
    let png: UTType = UTI::from_identifier(core_types::PNG).unwrap();
    let png_ref: UTTypeReference = png.clone();

    assert_eq!(png_ref.identifier(), core_types::PNG);
    assert_eq!(png_ref, png);
}

#[test]
fn filename_and_mime_mapping_helpers_match_tag_queries() {
    let image = UTI::from_identifier(core_types::IMAGE).unwrap();
    let jpeg = UTI::from_identifier(core_types::JPEG).unwrap();

    let by_extension = UTI::types_for_filename_extension("jpg", Some(&image)).unwrap();
    let by_mime = UTI::types_for_mime_type("image/jpeg", Some(&image)).unwrap();

    assert!(by_extension
        .iter()
        .any(|candidate| candidate.identifier() == core_types::JPEG));
    assert!(by_mime
        .iter()
        .any(|candidate| candidate.identifier() == core_types::JPEG));
    assert!(jpeg
        .preferred_filename_extension()
        .is_some_and(|value| jpeg.filename_extensions().contains(&value)));
    assert!(jpeg
        .preferred_mime_type()
        .is_some_and(|value| jpeg.mime_types().contains(&value)));
}

#[test]
fn ostype_helpers_round_trip_known_types() {
    let png_code = os_type::encode("PNGf").unwrap();
    let pdf_code = os_type::encode("PDF ").unwrap();
    let image = UTI::from_identifier(core_types::IMAGE).unwrap();

    assert_eq!(os_type::decode(png_code), "PNGf");
    assert_eq!(os_type::decode(pdf_code), "PDF ");
    assert_eq!(
        UTI::from_os_type(png_code).unwrap().identifier(),
        core_types::PNG
    );
    assert_eq!(
        UTI::from_os_type(pdf_code).unwrap().identifier(),
        core_types::PDF
    );

    let png = UTI::from_identifier(core_types::PNG).unwrap();
    assert_eq!(png.preferred_os_type_string().as_deref(), Some("PNGf"));
    assert!(png.os_types().contains(&png_code));

    let matches = UTI::types_for_os_type(png_code, Some(&image)).unwrap();
    assert!(matches
        .iter()
        .any(|candidate| candidate.identifier() == core_types::PNG));
}

#[test]
fn dynamic_and_local_types_behave_as_expected() {
    let dynamic = UTI::from_filename_extension("zzzzqwerty").unwrap();
    let data = UTI::from_identifier(core_types::DATA).unwrap();
    let exported =
        UTI::exported_type_with_identifier_conforming_to("com.example.uti-rs-exported", &data)
            .unwrap();
    let imported =
        UTI::imported_type_with_identifier_conforming_to("com.example.uti-rs-imported", &data)
            .unwrap();

    assert!(dynamic.is_dynamic());
    assert!(!dynamic.is_declared());
    assert!(dynamic.identifier().starts_with("dyn."));

    assert_eq!(exported.identifier(), "com.example.uti-rs-exported");
    assert_eq!(imported.identifier(), "com.example.uti-rs-imported");
    assert!(exported.is_declared());
    assert!(imported.is_declared());
    assert!(!exported.is_dynamic());
    assert!(!imported.is_dynamic());
}

#[test]
fn conformance_helpers_and_supertypes_cover_tree_queries() {
    let png = UTI::from_identifier(core_types::PNG).unwrap();
    let image = UTI::from_identifier(core_types::IMAGE).unwrap();
    let data = UTI::from_identifier(core_types::DATA).unwrap();
    let content = UTI::from_identifier(core_types::CONTENT).unwrap();
    let supertypes: BTreeSet<_> = png
        .supertypes()
        .into_iter()
        .map(|supertype| supertype.identifier())
        .collect();

    assert!(png.conforms_to(&image));
    assert!(png.conforms_to(&data));
    assert!(image.is_supertype_of(&png));
    assert!(png.is_subtype_of(&image));
    assert!(supertypes.contains(core_types::IMAGE));
    assert!(supertypes.contains(core_types::DATA));
    assert!(supertypes.contains(core_types::CONTENT));
    assert_eq!(
        png.version_number().map(|value| value.to_string()),
        png.version().map(|value| value.to_string())
    );
    assert!(png.is_public());
    assert!(content.is_public_type());
}

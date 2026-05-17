use std::collections::BTreeSet;

use uti::{core_types, UTI};

#[test]
fn conformance_relationships_hold_for_png_and_plain_text() {
    let png = UTI::from_identifier(core_types::PNG).unwrap();
    let image = UTI::from_identifier(core_types::IMAGE).unwrap();
    let data = UTI::from_identifier(core_types::DATA).unwrap();
    let text = UTI::from_identifier(core_types::TEXT).unwrap();
    let plain_text = UTI::from_identifier(core_types::PLAIN_TEXT).unwrap();
    let plain_text_supertypes: BTreeSet<_> = plain_text
        .supertypes()
        .into_iter()
        .map(|supertype| supertype.identifier())
        .collect();

    assert!(png.conforms_to(&image));
    assert!(png.conforms_to(&data));
    assert!(plain_text.conforms_to(&text));
    assert!(plain_text.conforms_to(&data));
    assert!(image.is_supertype_of(&png));
    assert!(text.is_supertype_of(&plain_text));
    assert!(png.is_subtype_of(&image));
    assert!(plain_text.is_subtype_of(&text));
    assert!(!png.conforms_to(&text));
    assert!(!text.conforms_to(&png));
    assert!(plain_text_supertypes.contains(core_types::TEXT));
    assert!(plain_text_supertypes.contains(core_types::DATA));
}

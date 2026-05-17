use std::collections::BTreeSet;

use uti::{core_types, tag_class, UTI};

#[test]
fn filename_extension_helpers_and_tag_values_match_for_jpeg() {
    let image = UTI::from_identifier(core_types::IMAGE).unwrap();
    let jpeg = UTI::from_identifier(core_types::JPEG).unwrap();
    let matches = UTI::types_for_filename_extension("jpg", Some(&image)).unwrap();
    let extensions: BTreeSet<_> = jpeg.filename_extensions().into_iter().collect();
    let tag_values: BTreeSet<_> = jpeg
        .tag_values(tag_class::FILENAME_EXTENSION)
        .into_iter()
        .collect();

    assert_eq!(
        UTI::from_filename_extension_conforming_to("jpg", &image).unwrap(),
        jpeg
    );
    assert_eq!(
        UTI::from_tag("jpg", tag_class::FILENAME_EXTENSION, Some(&image)).unwrap(),
        jpeg
    );
    assert!(matches.iter().any(|candidate| candidate == &jpeg));
    assert!(extensions.contains("jpg"));
    assert!(extensions.contains("jpeg"));
    assert_eq!(extensions, tag_values);
    assert!(jpeg
        .preferred_filename_extension()
        .is_some_and(|value| extensions.contains(&value)));
}

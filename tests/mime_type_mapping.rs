use std::collections::BTreeSet;

use uti::{core_types, tag_class, UTI};

#[test]
fn mime_type_helpers_and_tag_values_match_for_json() {
    let data = UTI::from_identifier(core_types::DATA).unwrap();
    let json = UTI::from_identifier(core_types::JSON).unwrap();
    let matches = UTI::types_for_mime_type("application/json", Some(&data)).unwrap();
    let mime_types: BTreeSet<_> = json.mime_types().into_iter().collect();
    let tag_values: BTreeSet<_> = json.tag_values(tag_class::MIME_TYPE).into_iter().collect();

    assert_eq!(UTI::from_mime_type("application/json").unwrap(), json);
    assert_eq!(
        UTI::from_mime_type_conforming_to("application/json", &data).unwrap(),
        json
    );
    assert_eq!(
        UTI::from_tag("application/json", tag_class::MIME_TYPE, Some(&data)).unwrap(),
        json
    );
    assert!(matches.iter().any(|candidate| candidate == &json));
    assert!(mime_types.contains("application/json"));
    assert_eq!(mime_types, tag_values);
    assert!(json
        .preferred_mime_type()
        .is_some_and(|value| mime_types.contains(&value)));
}

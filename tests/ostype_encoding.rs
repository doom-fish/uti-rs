use uti::{core_types, os_type, tag_class, UTIError, UTI};

#[test]
fn ostype_helpers_round_trip_pngf_and_find_public_png() {
    let image = UTI::from_identifier(core_types::IMAGE).unwrap();
    let png = UTI::from_identifier(core_types::PNG).unwrap();
    let png_code = os_type::encode("PNGf").unwrap();

    assert_eq!(png_code, os_type::encode_bytes([b'P', b'N', b'G', b'f']));
    assert_eq!(os_type::decode_bytes(png_code), [b'P', b'N', b'G', b'f']);
    assert_eq!(os_type::decode(png_code), "PNGf");
    assert_eq!(UTI::from_os_type(png_code).unwrap(), png);
    assert_eq!(
        UTI::from_os_type_conforming_to(png_code, &image).unwrap(),
        png
    );
    assert_eq!(
        UTI::from_tag("PNGf", tag_class::OS_TYPE, Some(&image)).unwrap(),
        png
    );
    assert!(UTI::types_for_os_type(png_code, Some(&image))
        .unwrap()
        .iter()
        .any(|candidate| candidate == &png));
    assert_eq!(png.preferred_os_type(), Some(png_code));
    assert!(matches!(
        os_type::encode("PNG"),
        Err(UTIError::InvalidArgument(_))
    ));
}

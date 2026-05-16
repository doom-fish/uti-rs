//! Demonstrate tag-class lookups, tag dictionaries, supertypes, and local types.
//!
//! Run: `cargo run --example 03_tags`

use uti::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let png = UTI::from_tag("png", tag_class::FILENAME_EXTENSION, None)?;
    let image = UTI::well_known("image").unwrap();
    let data = UTI::well_known("data").unwrap();
    let png_types = UTI::types_with_tag("png", tag_class::FILENAME_EXTENSION, Some(&image))?;
    let imported =
        UTI::imported_type_with_identifier_conforming_to("com.example.demo-json", &data)?;

    println!("png identifier: {}", png.identifier());
    println!("png conforms to image: {}", png.conforms_to(&image));
    println!(
        "png tag classes: {:?}",
        png.tags().keys().cloned().collect::<Vec<_>>()
    );
    println!(
        "png supertypes: {:?}",
        png.supertypes()
            .into_iter()
            .map(|supertype| supertype.identifier())
            .collect::<Vec<_>>()
    );
    println!(
        "matching png image types: {:?}",
        png_types.iter().map(UTI::identifier).collect::<Vec<_>>()
    );
    println!("imported local identifier: {}", imported.identifier());
    Ok(())
}

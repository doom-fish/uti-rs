//! Demonstrate dynamic UTIs and classic `OSType` / `FourCharCode` helpers.
//!
//! Run: `cargo run --example 06_dynamic_ostype`

use uti::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let image = UTI::well_known("image").unwrap();
    let png_code = os_type::encode("PNGf")?;
    let png = UTI::from_os_type(png_code)?;
    let dynamic = UTI::from_filename_extension("zzzzqwerty")?;
    let jpeg_matches = UTI::types_for_filename_extension("jpg", Some(&image))?;
    let mime_matches = UTI::types_for_mime_type("image/jpeg", Some(&image))?;

    println!("PNGf => {}", png.identifier());
    println!(
        "png os types: {:?}",
        png.os_types()
            .into_iter()
            .map(os_type::decode)
            .collect::<Vec<_>>()
    );
    println!(
        "jpg image types: {:?}",
        jpeg_matches.iter().map(UTI::identifier).collect::<Vec<_>>()
    );
    println!(
        "image/jpeg types: {:?}",
        mime_matches.iter().map(UTI::identifier).collect::<Vec<_>>()
    );
    println!(
        "dynamic {} (is_dynamic={}, declared={})",
        dynamic.identifier(),
        dynamic.is_dynamic(),
        dynamic.is_declared()
    );
    Ok(())
}

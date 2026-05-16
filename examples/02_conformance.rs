//! Demonstrate UTI conformance: PNG → image → data hierarchy.
//!
//! Run: `cargo run --example 02_conformance`

use uti::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let png = UTI::from_filename_extension("png")?;
    let jpeg = UTI::from_filename_extension("jpg")?;
    let pdf = UTI::from_filename_extension("pdf")?;
    let mp3 = UTI::from_filename_extension("mp3")?;
    let html = UTI::from_filename_extension("html")?;

    let image = UTI::well_known("image").unwrap();
    let audio = UTI::well_known("audio").unwrap();
    let text = UTI::well_known("text").unwrap();
    let data = UTI::well_known("data").unwrap();
    let content = UTI::well_known("content").unwrap();

    println!("Type-conformance matrix:");
    println!(
        "{:<8} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "type", "image?", "audio?", "text?", "data?", "content?"
    );
    println!("{}", "-".repeat(64));
    for (name, t) in &[
        ("png", &png),
        ("jpg", &jpeg),
        ("pdf", &pdf),
        ("mp3", &mp3),
        ("html", &html),
    ] {
        println!(
            "{:<8} {:>10} {:>10} {:>10} {:>10} {:>10}",
            name,
            t.conforms_to(&image),
            t.conforms_to(&audio),
            t.conforms_to(&text),
            t.conforms_to(&data),
            t.conforms_to(&content),
        );
    }

    assert!(png.conforms_to(&image), "PNG should conform to image");
    assert!(mp3.conforms_to(&audio), "MP3 should conform to audio");
    assert!(html.conforms_to(&text), "HTML should conform to text");
    assert!(png.conforms_to(&data), "PNG should conform to data");
    println!("\nOK All conformance checks passed");
    Ok(())
}

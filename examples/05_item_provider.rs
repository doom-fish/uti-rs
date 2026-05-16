//! Demonstrate `NSItemProvider` integration using typed `UTI` content types.
//!
//! Run: `cargo run --example 05_item_provider`

use std::path::Path;

use uti::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = ItemProvider::new();
    let plain_text = UTI::well_known("plainText").unwrap();
    let text = UTI::well_known("text").unwrap();

    provider.register_data_representation(
        &plain_text,
        RepresentationVisibility::OwnProcess,
        b"hello from item provider",
    );
    provider.register_file_representation(
        &text,
        RepresentationVisibility::OwnProcess,
        false,
        "README.md",
        false,
    )?;

    let registered = provider
        .registered_content_types()?
        .into_iter()
        .map(|content_type| content_type.identifier())
        .collect::<Vec<_>>();
    let text_like = provider
        .registered_content_types_conforming_to(&text)?
        .into_iter()
        .map(|content_type| content_type.identifier())
        .collect::<Vec<_>>();
    let loaded_text = String::from_utf8(provider.load_data_representation(&plain_text)?)?;
    let loaded_file = provider.load_file_representation(&text, false)?;

    println!("registered content types: {registered:?}");
    println!("text-like content types: {text_like:?}");
    println!("loaded text: {loaded_text}");
    println!(
        "loaded file exists: {} ({})",
        Path::new(&loaded_file.path).exists(),
        loaded_file.path
    );
    Ok(())
}

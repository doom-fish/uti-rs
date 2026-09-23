//! Demonstrate `NSItemProvider` integration using typed `UTI` content types.
//!
//! Run: `cargo run --example 05_item_provider`

use std::thread;

use uti::prelude::*;

fn run() -> Result<(), UTIError> {
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
    let loaded_text =
        String::from_utf8_lossy(&provider.load_data_representation(&plain_text)?).into_owned();

    let destination = std::env::temp_dir().join(format!("uti-example-{}", std::process::id()));
    std::fs::create_dir_all(&destination)
        .map_err(|error| UTIError::OperationFailed(error.to_string()))?;
    let loaded_file = provider.load_file_representation(&text, false, &destination)?;

    println!("registered content types: {registered:?}");
    println!("text-like content types: {text_like:?}");
    println!("loaded text: {loaded_text}");
    println!(
        "kept copy exists: {} ({})",
        loaded_file.path.exists(),
        loaded_file.path.display()
    );
    std::fs::remove_dir_all(&destination)
        .map_err(|error| UTIError::OperationFailed(error.to_string()))?;
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    thread::spawn(run)
        .join()
        .map_err(|_| "item provider thread panicked")??;
    Ok(())
}

//! Demonstrate executor-agnostic async `NSItemProvider` loading.
//!
//! Run: `cargo run --example 07_item_provider_async --features async`

use std::path::Path;

use uti::{async_api::AsyncItemProvider, prelude::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    pollster::block_on(async {
        let provider = ItemProvider::new();
        let plain_text = UTI::well_known("plainText").unwrap();
        let text = UTI::well_known("text").unwrap();

        provider.register_data_representation(
            &plain_text,
            RepresentationVisibility::OwnProcess,
            b"hello from async item provider",
        );
        provider.register_file_representation(
            &text,
            RepresentationVisibility::OwnProcess,
            false,
            "README.md",
            false,
        )?;

        let async_provider = AsyncItemProvider::new(&provider);
        let loaded_text =
            String::from_utf8(async_provider.load_data_representation(&plain_text).await?)?;
        let loaded_file = provider
            .load_file_representation_async(&text, false)
            .await?;

        println!("loaded text: {loaded_text}");
        println!(
            "loaded file exists: {} ({})",
            Path::new(&loaded_file.path).exists(),
            loaded_file.path
        );
        Ok(())
    })
}

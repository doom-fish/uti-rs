//! Demonstrate executor-agnostic async `NSItemProvider` loading.
//!
//! Run: `cargo run --example 07_item_provider_async --features async`

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

        let destination =
            std::env::temp_dir().join(format!("uti-async-example-{}", std::process::id()));
        std::fs::create_dir_all(&destination)?;
        let loaded_file = provider
            .load_file_representation_async(&text, false, &destination)
            .await?;

        println!("loaded text: {loaded_text}");
        println!(
            "kept copy exists: {} ({})",
            loaded_file.path.exists(),
            loaded_file.path.display()
        );
        std::fs::remove_dir_all(&destination)?;
        Ok(())
    })
}

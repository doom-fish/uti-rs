#![cfg(feature = "async")]

use std::path::Path;

use uti::{async_api::AsyncItemProvider, prelude::*};

#[test]
fn async_item_provider_loads_registered_representations() -> Result<(), Box<dyn std::error::Error>>
{
    let provider = ItemProvider::new();
    let plain_text = UTI::well_known("plainText").unwrap();
    let text = UTI::well_known("text").unwrap();
    let payload = b"hello from async item provider".to_vec();

    provider.register_data_representation(
        &plain_text,
        RepresentationVisibility::OwnProcess,
        &payload,
    );
    provider.register_file_representation(
        &text,
        RepresentationVisibility::OwnProcess,
        false,
        "README.md",
        false,
    )?;

    pollster::block_on(async {
        let loaded_bytes = AsyncItemProvider::new(&provider)
            .load_data_representation(&plain_text)
            .await?;
        assert_eq!(loaded_bytes, payload);

        let loaded_file = provider
            .load_file_representation_async(&text, false)
            .await?;
        assert!(Path::new(&loaded_file.path).exists());
        Ok::<(), UTIError>(())
    })?;

    Ok(())
}

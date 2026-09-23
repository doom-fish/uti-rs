#![cfg(feature = "async")]

use std::path::PathBuf;
use std::thread;
use std::time::Duration;

use uti::{async_api::AsyncItemProvider, prelude::*};

fn fresh_directory(name: &str) -> PathBuf {
    let directory =
        PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).unwrap();
    directory
}

#[test]
fn async_item_provider_loads_registered_representations() -> Result<(), Box<dyn std::error::Error>>
{
    let provider = ItemProvider::new();
    let plain_text = UTI::well_known("plainText").unwrap();
    let text = UTI::well_known("text").unwrap();
    let payload = b"hello from async item provider".to_vec();
    let destination = fresh_directory("async-item-provider");

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
            .load_file_representation_async(&text, false, &destination)
            .await?;
        assert!(!loaded_file.open_in_place);
        assert_eq!(loaded_file.path.parent(), Some(destination.as_path()));
        thread::sleep(Duration::from_millis(300));
        assert_eq!(
            std::fs::read(&loaded_file.path).unwrap(),
            std::fs::read("README.md").unwrap()
        );
        Ok::<(), UTIError>(())
    })?;

    std::fs::remove_dir_all(&destination)?;
    Ok(())
}

#[test]
fn async_file_load_rejects_a_non_utf8_destination() {
    use std::ffi::OsStr;
    use std::os::unix::ffi::OsStrExt;

    let provider = ItemProvider::new();
    let text = UTI::well_known("text").unwrap();
    let destination = std::path::Path::new(OsStr::from_bytes(b"/nonexistent/\xff"));

    let error =
        pollster::block_on(provider.load_file_representation_async(&text, false, destination))
            .unwrap_err();
    assert!(matches!(error, UTIError::InvalidArgument(_)), "{error}");
}

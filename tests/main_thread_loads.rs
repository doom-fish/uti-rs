use uti::{core_types, ItemProvider, RepresentationVisibility, UTIError, UTI};

fn main() {
    let text = UTI::from_identifier(core_types::PLAIN_TEXT).unwrap();
    let provider = ItemProvider::new();
    provider.register_data_representation(&text, RepresentationVisibility::OwnProcess, b"main");

    let data = provider.load_data_representation(&text).unwrap_err();
    assert!(
        matches!(&data, UTIError::OperationFailed(message) if message.contains("main thread")),
        "{data}"
    );

    let file = provider
        .load_file_representation(&text, false, env!("CARGO_TARGET_TMPDIR"))
        .unwrap_err();
    assert!(
        matches!(&file, UTIError::OperationFailed(message) if message.contains("main thread")),
        "{file}"
    );

    let loaded = std::thread::spawn(move || {
        let provider = ItemProvider::new();
        provider.register_data_representation(
            &text,
            RepresentationVisibility::OwnProcess,
            b"worker",
        );
        provider.load_data_representation(&text)
    })
    .join()
    .unwrap()
    .unwrap();
    assert_eq!(loaded, b"worker");

    println!("main-thread load checks passed");
}

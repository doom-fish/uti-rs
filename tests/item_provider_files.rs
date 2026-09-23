use std::ffi::OsStr;
use std::os::unix::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

use uti::{additions, core_types, ItemProvider, RepresentationVisibility, UTIError, UTI};

fn fresh_directory(name: &str) -> PathBuf {
    let directory =
        PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&directory);
    std::fs::create_dir_all(&directory).unwrap();
    directory
}

fn non_utf8_path() -> &'static Path {
    Path::new(OsStr::from_bytes(b"/nonexistent/\xff\xfe"))
}

#[test]
fn copied_file_survives_after_the_completion_handler_returns() {
    let source_directory = fresh_directory("file-load-source");
    let destination = fresh_directory("file-load-destination");
    let source = source_directory.join("notes.txt");
    std::fs::write(&source, b"kept after the handler").unwrap();
    let text = UTI::from_identifier(core_types::PLAIN_TEXT).unwrap();
    let provider = ItemProvider::new();
    provider
        .register_file_representation(
            &text,
            RepresentationVisibility::OwnProcess,
            false,
            &source,
            false,
        )
        .unwrap();

    let loaded = provider
        .load_file_representation(&text, false, &destination)
        .unwrap();

    assert!(!loaded.open_in_place);
    assert_eq!(loaded.path, destination.join("notes.txt"));
    thread::sleep(Duration::from_millis(500));
    assert_eq!(
        std::fs::read(&loaded.path).unwrap(),
        b"kept after the handler"
    );
    assert_eq!(std::fs::read(&source).unwrap(), b"kept after the handler");

    std::fs::remove_dir_all(&source_directory).unwrap();
    std::fs::remove_dir_all(&destination).unwrap();
}

#[test]
fn data_representation_loaded_as_a_file_is_kept() {
    let destination = fresh_directory("data-as-file");
    let text = UTI::from_identifier(core_types::PLAIN_TEXT).unwrap();
    let provider = ItemProvider::new();
    provider.register_data_representation(&text, RepresentationVisibility::OwnProcess, b"bytes");

    let loaded = provider
        .load_file_representation(&text, false, &destination)
        .unwrap();

    assert_eq!(loaded.path.parent(), Some(destination.as_path()));
    thread::sleep(Duration::from_millis(300));
    assert_eq!(std::fs::read(&loaded.path).unwrap(), b"bytes");

    let second = provider
        .load_file_representation(&text, false, &destination)
        .unwrap_err();
    assert!(matches!(second, UTIError::OperationFailed(_)), "{second}");
    assert_eq!(std::fs::read(&loaded.path).unwrap(), b"bytes");

    std::fs::remove_dir_all(&destination).unwrap();
}

#[test]
fn file_load_into_a_missing_directory_fails() {
    let text = UTI::from_identifier(core_types::PLAIN_TEXT).unwrap();
    let provider = ItemProvider::new();
    provider.register_data_representation(&text, RepresentationVisibility::OwnProcess, b"bytes");

    let error = provider
        .load_file_representation(&text, false, "/nonexistent/uti-rs-destination")
        .unwrap_err();
    assert!(matches!(error, UTIError::OperationFailed(_)), "{error}");
}

#[test]
fn in_place_loads_return_the_registered_path_unchanged() {
    let directory = fresh_directory("in-place");
    let source = directory.join("cafe\u{301} \u{65e5}\u{672c} #1.txt");
    std::fs::write(&source, b"in place").unwrap();
    let text = UTI::from_identifier(core_types::PLAIN_TEXT).unwrap();
    let provider = ItemProvider::new();
    provider
        .register_file_representation(
            &text,
            RepresentationVisibility::OwnProcess,
            true,
            &source,
            false,
        )
        .unwrap();

    let loaded = provider
        .load_file_representation(&text, true, "/nonexistent/unused")
        .unwrap();

    assert!(loaded.open_in_place);
    assert_eq!(
        loaded.path.as_os_str().as_bytes(),
        source.as_os_str().as_bytes()
    );
    std::fs::remove_dir_all(&directory).unwrap();
}

#[test]
fn non_utf8_paths_are_rejected_instead_of_rewritten() {
    let text = UTI::from_identifier(core_types::PLAIN_TEXT).unwrap();
    let provider = ItemProvider::new();

    let from_path = ItemProvider::from_file_path(
        non_utf8_path(),
        Some(&text),
        false,
        false,
        RepresentationVisibility::OwnProcess,
    )
    .unwrap_err();
    assert!(
        matches!(from_path, UTIError::InvalidArgument(_)),
        "{from_path}"
    );

    let register = provider
        .register_file_representation(
            &text,
            RepresentationVisibility::OwnProcess,
            false,
            non_utf8_path(),
            false,
        )
        .unwrap_err();
    assert!(
        matches!(register, UTIError::InvalidArgument(_)),
        "{register}"
    );

    let load = provider
        .load_file_representation(&text, false, non_utf8_path())
        .unwrap_err();
    assert!(matches!(load, UTIError::InvalidArgument(_)), "{load}");

    let png = UTI::from_identifier(core_types::PNG).unwrap();
    let append = additions::append_path_extension_for_type(non_utf8_path(), &png).unwrap_err();
    assert!(matches!(append, UTIError::InvalidArgument(_)), "{append}");
}

#[test]
fn path_helpers_keep_unicode_bytes() {
    let png = UTI::from_identifier(core_types::PNG).unwrap();
    let base = Path::new("/tmp/cafe\u{301} \u{65e5}\u{672c}");

    let with_extension = additions::append_path_extension_for_type(base, &png).unwrap();
    assert_eq!(
        with_extension.as_os_str().as_bytes(),
        "/tmp/cafe\u{301} \u{65e5}\u{672c}.png".as_bytes()
    );

    let component = additions::append_path_component_conforming_to(base, "shot", &png).unwrap();
    assert_eq!(
        component.as_os_str().as_bytes(),
        "/tmp/cafe\u{301} \u{65e5}\u{672c}/shot.png".as_bytes()
    );
}

#[test]
fn registered_content_types_decode_from_the_bridge() {
    let provider = ItemProvider::new();
    let png = UTI::from_identifier(core_types::PNG).unwrap();
    let text = UTI::from_identifier(core_types::PLAIN_TEXT).unwrap();
    provider.register_data_representation(&png, RepresentationVisibility::OwnProcess, b"png");
    provider.register_data_representation(&text, RepresentationVisibility::OwnProcess, b"txt");

    let registered = provider.registered_content_types().unwrap();
    assert_eq!(registered, vec![png, text]);
}

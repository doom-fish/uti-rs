use std::collections::BTreeMap;
use std::path::PathBuf;

use uti::{core_types, UTI};

const RUNTIME_SOURCED: [&str; 5] = ["bmp", "ico", "wav", "exe", "internetShortcut"];

const IRREGULAR_NAMES: [(&str, &str); 5] = [
    ("webP", "WEBP"),
    ("sceneKitScene", "SCENEKIT_SCENE"),
    ("quickTimeMovie", "QUICKTIME_MOVIE"),
    ("vCard", "VCARD"),
    ("geoJSON", "GEOJSON"),
];

fn read(path: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path);
    std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read {}: {error}", path.display()))
}

fn well_known_names() -> Vec<String> {
    let bridge = read("swift-bridge/Sources/UTIBridge/UTI.swift");
    let start = bridge.find("private let wellKnownIdentifiers").unwrap();
    let table = &bridge[start..];
    let table = &table[..table.find("\n]").unwrap()];
    regex_lite::Regex::new(r#"(?m)^\s*"([A-Za-z0-9]+)":"#)
        .unwrap()
        .captures_iter(table)
        .map(|captures| captures[1].to_owned())
        .collect()
}

fn core_type_constants() -> BTreeMap<String, String> {
    regex_lite::Regex::new(r#"pub const ([A-Z0-9_]+): &str = "([^"]+)";"#)
        .unwrap()
        .captures_iter(&read("src/core_types.rs"))
        .map(|captures| (captures[1].to_owned(), captures[2].to_owned()))
        .collect()
}

fn constant_name(name: &str) -> String {
    if let Some((_, constant)) = IRREGULAR_NAMES.iter().find(|(key, _)| *key == name) {
        return (*constant).to_owned();
    }
    let characters: Vec<char> = name.chars().collect();
    let mut out = String::new();
    for (index, character) in characters.iter().enumerate() {
        if index > 0 && character.is_ascii_uppercase() {
            let previous = characters[index - 1];
            let next_is_lower = characters
                .get(index + 1)
                .is_some_and(char::is_ascii_lowercase);
            if previous.is_ascii_lowercase()
                || previous.is_ascii_digit()
                || (previous.is_ascii_uppercase() && next_is_lower)
            {
                out.push('_');
            }
        }
        out.push(character.to_ascii_uppercase());
    }
    out
}

#[test]
fn every_well_known_name_matches_its_core_type_constant() {
    let constants = core_type_constants();
    let names = well_known_names();
    assert!(names.len() > 100, "parsed only {} names", names.len());

    for name in &names {
        let uti = UTI::well_known(name).unwrap_or_else(|| panic!("{name} did not resolve"));
        assert!(uti.is_declared(), "{name} resolved to an undeclared type");
        let constant = constant_name(name);
        match constants.get(&constant) {
            Some(value) => assert_eq!(&uti.identifier(), value, "{name} vs core_types::{constant}"),
            None => assert!(
                RUNTIME_SOURCED.contains(&name.as_str()),
                "{name} has no core_types::{constant}"
            ),
        }
    }
}

#[test]
fn corrected_identifiers_match_the_framework() {
    let apple_script = UTI::well_known("appleScript").unwrap();
    assert_eq!(apple_script.identifier(), core_types::APPLE_SCRIPT);
    assert_ne!(core_types::APPLE_SCRIPT, core_types::OSA_SCRIPT);

    let calendar_event = UTI::well_known("calendarEvent").unwrap();
    assert_eq!(calendar_event.identifier(), core_types::CALENDAR_EVENT);

    let shortcut = UTI::well_known("internetShortcut").unwrap();
    let location = UTI::well_known("internetLocation").unwrap();
    assert_eq!(location.identifier(), core_types::INTERNET_LOCATION);
    assert_ne!(shortcut, location);
    assert_eq!(
        shortcut.preferred_filename_extension().as_deref(),
        Some("url")
    );
}

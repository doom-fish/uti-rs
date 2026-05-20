//! API-surface coverage harness for `uti`.
//!
//! `UniformTypeIdentifiers` is an Obj-C / Swift framework. We compare the
//! public headers against the Rust + Swift bridge surface exposed by this
//! crate.

#![allow(clippy::cast_precision_loss, clippy::iter_on_single_items)]

use std::collections::BTreeSet;
use std::path::PathBuf;
use std::process::Command;

fn sdk_root() -> PathBuf {
    let out = Command::new("xcrun")
        .args(["--sdk", "macosx", "--show-sdk-path"])
        .output()
        .expect("xcrun");
    assert!(out.status.success());
    PathBuf::from(String::from_utf8(out.stdout).unwrap().trim().to_string())
}

fn read(path: &PathBuf) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn manifest_file(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path)
}

fn read_bridge() -> String {
    let bridge_dir = manifest_file("swift-bridge/Sources/UTIBridge");
    let mut files = std::fs::read_dir(&bridge_dir)
        .unwrap_or_else(|e| panic!("read_dir {}: {e}", bridge_dir.display()))
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "swift"))
        .collect::<Vec<_>>();
    files.sort();
    files
        .into_iter()
        .map(|path| read(&path))
        .collect::<Vec<_>>()
        .join("\n")
}

fn read_rust(path: &str) -> String {
    read(&manifest_file(path))
}

fn read_header(name: &str) -> String {
    read(&sdk_root().join(format!(
        "System/Library/Frameworks/UniformTypeIdentifiers.framework/Headers/{name}.h"
    )))
}

fn extract_interfaces(header: &str, type_name: &str) -> Vec<String> {
    let needle = format!("@interface {type_name}");
    let mut rest = header;
    let mut out = Vec::new();
    while let Some(start) = rest.find(&needle) {
        let after_start = &rest[start..];
        let Some(end_off) = after_start.find("@end") else {
            break;
        };
        out.push(after_start[..end_off].to_string());
        rest = &after_start[end_off + 4..];
    }
    out
}

fn extract_member_surface(body: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let method_re =
        regex_lite::Regex::new(r"(?m)^\s*[+\-]\s*\([^\)]*\)\s*([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    for captures in method_re.captures_iter(body) {
        out.insert(captures[1].to_string());
    }
    let prop_re = regex_lite::Regex::new(
        r"(?m)^\s*@property\s*(?:\([^\)]*\))?\s*[^;]*?\b([A-Za-z_][A-Za-z0-9_]*)\s*(?:NS_|API_|;)",
    )
    .unwrap();
    for captures in prop_re.captures_iter(body) {
        out.insert(captures[1].to_string());
    }
    let getter_re = regex_lite::Regex::new(r"getter\s*=\s*([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    for captures in getter_re.captures_iter(body) {
        out.insert(captures[1].to_string());
    }
    out
}

fn references_in_bridge(symbols: &BTreeSet<String>) -> BTreeSet<String> {
    let bridge = read_bridge();
    let aliases = swift_aliases();
    symbols
        .iter()
        .filter(|name| {
            let pattern = format!(r"\b{}\b", regex_lite::escape(name));
            if regex_lite::Regex::new(&pattern).unwrap().is_match(&bridge) {
                return true;
            }
            if let Some(form) = aliases.get(name.as_str()) {
                return bridge.contains(form);
            }
            false
        })
        .cloned()
        .collect()
}

fn swift_aliases() -> std::collections::BTreeMap<&'static str, &'static str> {
    [
        ("typeWithIdentifier", "UTType(String"),
        ("typeWithFilenameExtension", "UTType(filenameExtension:"),
        ("typeWithMIMEType", "UTType(mimeType:"),
        ("typeWithTag", "UTType(tag:"),
        ("typesWithTag", "UTType.types("),
        ("conformsToType", ".conforms(to:"),
        ("isSupertypeOfType", ".isSupertype(of:"),
        ("isSubtypeOfType", ".isSubtype(of:"),
        ("preferredFilenameExtension", "preferredFilenameExtension"),
        ("preferredMIMEType", "preferredMIMEType"),
        ("localizedDescription", "localizedDescription"),
        ("version", "uti_version"),
        ("referenceURL", "referenceURL"),
        ("dynamic", "uti_is_dynamic"),
        ("isDynamic", "isDynamic"),
        ("declared", "uti_is_declared"),
        ("isDeclared", "isDeclared"),
        ("publicType", "uti_is_public_type"),
        ("isPublicType", "is_public_type"),
        ("supertypes", ".supertypes"),
        ("tags", "uti_tags"),
        ("exportedTypeWithIdentifier", "UTType(exportedAs:"),
        ("importedTypeWithIdentifier", "UTType(importedAs:"),
    ]
    .into_iter()
    .collect()
}

fn report(
    name: &str,
    apple: &BTreeSet<String>,
    ours: &BTreeSet<String>,
    omitted: &BTreeSet<String>,
) {
    let wrapped: BTreeSet<&String> = apple.intersection(ours).collect();
    let missing: BTreeSet<&String> = apple
        .difference(ours)
        .filter(|symbol| !omitted.contains(*symbol))
        .collect();
    let coverable = wrapped.len() + missing.len();
    let pct = if coverable == 0 {
        100.0
    } else {
        wrapped.len() as f64 / coverable as f64 * 100.0
    };
    println!(
        "\n=== {name} ===\n  apple={}, omitted={}, coverable={coverable}, wrapped={}, missing={}, pct={pct:.1}%",
        apple.len(),
        omitted.len(),
        wrapped.len(),
        missing.len(),
    );
    if !missing.is_empty() {
        for symbol in &missing {
            println!("  - {symbol}");
        }
    }
    assert!(pct >= 100.0, "{name}: {pct:.1}%");
}

fn omitted_set<const N: usize>(items: [&str; N]) -> BTreeSet<String> {
    items.into_iter().map(String::from).collect()
}

fn decode_string_literal_escapes(value: &str) -> String {
    let mut chars = value.chars();
    let mut out = String::new();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        match chars.next().expect("escape suffix") {
            '\\' => out.push('\\'),
            '"' => out.push('"'),
            'n' => out.push('\n'),
            'r' => out.push('\r'),
            't' => out.push('\t'),
            '0' => out.push('\0'),
            'x' => {
                let hi = chars.next().expect("hex escape high nibble");
                let lo = chars.next().expect("hex escape low nibble");
                let byte = u8::from_str_radix(&format!("{hi}{lo}"), 16).expect("valid hex escape");
                out.push(char::from(byte));
            }
            'u' => {
                assert_eq!(chars.next(), Some('{'));
                let mut code_point = String::new();
                loop {
                    let next = chars.next().expect("unicode escape terminator");
                    if next == '}' {
                        break;
                    }
                    code_point.push(next);
                }
                let value = u32::from_str_radix(&code_point, 16).expect("valid unicode escape");
                out.push(char::from_u32(value).expect("valid unicode scalar"));
            }
            other => panic!("unsupported string escape: {other}"),
        }
    }
    out
}

fn extract_utcoretypes_identifiers() -> BTreeSet<String> {
    let header = read_header("UTCoreTypes");
    let mut current_identifier = None;
    let mut identifiers = BTreeSet::new();
    let uti_re = regex_lite::Regex::new(r"UTI:\s*([^\s]+)").unwrap();
    let const_re = regex_lite::Regex::new(r"UT_EXPORT UTType \*const UTType\w+").unwrap();
    for line in header.lines() {
        if let Some(captures) = uti_re.captures(line) {
            current_identifier = Some(captures[1].to_string());
        }
        if const_re.is_match(line) {
            if let Some(identifier) = current_identifier.take() {
                identifiers.insert(identifier);
            }
        }
    }
    identifiers
}

#[test]
fn ut_type_primary_interface_coverage() {
    let interfaces = extract_interfaces(&read_header("UTType"), "UTType");
    let apple = extract_member_surface(&interfaces[0]);
    let ours = references_in_bridge(&apple);
    let omitted = omitted_set(["init", "new"]);
    report("UTType", &apple, &ours, &omitted);
}

#[test]
fn ut_type_conformance_category_coverage() {
    let interfaces = extract_interfaces(&read_header("UTType"), "UTType");
    let apple = extract_member_surface(&interfaces[1]);
    let ours = references_in_bridge(&apple);
    report("UTType (Conformance)", &apple, &ours, &BTreeSet::new());
}

#[test]
fn ut_type_tag_specification_coverage() {
    let interfaces = extract_interfaces(&read_header("UTType"), "UTType");
    let apple = extract_member_surface(&interfaces[2]);
    let ours = references_in_bridge(&apple);
    report(
        "UTType (UTTagSpecification)",
        &apple,
        &ours,
        &BTreeSet::new(),
    );
}

#[test]
fn ut_type_local_constants_coverage() {
    let interfaces = extract_interfaces(&read_header("UTType"), "UTType");
    let apple = extract_member_surface(&interfaces[3]);
    let ours = references_in_bridge(&apple);
    report("UTType (LocalConstants)", &apple, &ours, &BTreeSet::new());
}

#[test]
fn ut_tag_class_constants_present() {
    let source = read_rust("src/tag_class.rs");
    assert!(source.contains("public.filename-extension"));
    assert!(source.contains("public.mime-type"));
    assert!(source.contains("com.apple.ostype"));
}

#[test]
fn ut_type_reference_alias_present() {
    let source = read_rust("src/uttype/mod.rs");
    assert!(source.contains("pub type UTType = UTI;"));
    assert!(source.contains("pub type UTTypeReference = UTI;"));
}

#[test]
fn ut_additions_bridge_coverage() {
    let bridge = read_bridge();
    let rust = read_rust("src/additions.rs");
    for symbol in [
        "uti_string_appending_path_component_conforming_to",
        "uti_string_appending_path_extension_for_type",
        "uti_url_appending_path_component_conforming_to",
        "uti_url_appending_path_extension_for_type",
    ] {
        assert!(bridge.contains(symbol), "bridge missing {symbol}");
    }
    for symbol in [
        "append_path_component_conforming_to",
        "append_path_extension_for_type",
        "append_url_path_component_conforming_to",
        "append_url_path_extension_for_type",
    ] {
        assert!(
            rust.contains(symbol),
            "Rust additions surface missing {symbol}"
        );
    }
}

#[test]
fn nsitemprovider_uttype_bridge_coverage() {
    let bridge = read_bridge();
    let rust = format!(
        "{}\n{}",
        read_rust("src/item_provider.rs"),
        read_rust("src/async_api.rs")
    );
    for symbol in [
        "item_provider_from_file_path",
        "item_provider_register_data_representation",
        "item_provider_register_file_representation",
        "item_provider_registered_type_identifiers",
        "item_provider_load_data_representation",
        "item_provider_load_file_representation",
        "item_provider_load_data_representation_async",
        "item_provider_load_file_representation_async",
    ] {
        assert!(bridge.contains(symbol), "bridge missing {symbol}");
    }
    for symbol in [
        "from_file_path",
        "register_data_representation",
        "register_file_representation",
        "registered_content_types",
        "load_data_representation",
        "load_file_representation",
        "load_data_representation_async",
        "load_file_representation_async",
        "AsyncItemProvider",
    ] {
        assert!(
            rust.contains(symbol),
            "Rust item-provider surface missing {symbol}"
        );
    }
}

#[test]
fn ut_core_types_identifier_coverage() {
    let apple = extract_utcoretypes_identifiers();
    let rust = read_rust("src/core_types.rs");
    let rust_values: BTreeSet<String> =
        regex_lite::Regex::new(r#"pub const [A-Z0-9_]+: &str = \"([^\"]+)\";"#)
            .unwrap()
            .captures_iter(&rust)
            .map(|captures| decode_string_literal_escapes(&captures[1]))
            .collect();
    let missing: BTreeSet<String> = apple.difference(&rust_values).cloned().collect();
    assert!(
        missing.is_empty(),
        "missing UTCoreTypes identifiers: {missing:?}"
    );
}

#[test]
fn ut_core_types_well_known_coverage() {
    let apple = extract_utcoretypes_identifiers();
    let bridge = read_bridge();
    let well_known_values: BTreeSet<String> =
        regex_lite::Regex::new(r#"(?m)^\s*\"[^\"]+\": \"([^\"]+)\","#)
            .unwrap()
            .captures_iter(&bridge)
            .map(|captures| decode_string_literal_escapes(&captures[1]))
            .collect();
    let missing: BTreeSet<String> = apple.difference(&well_known_values).cloned().collect();
    assert!(
        missing.is_empty(),
        "missing well_known identifiers: {missing:?}"
    );
}

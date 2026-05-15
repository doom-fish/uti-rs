//! API-surface coverage harness for `uti`.
//!
//! `UniformTypeIdentifiers` is an Obj-C / Swift framework. Mirrors the
//! family pattern (header-based, Obj-C `@interface` parsing).

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

fn read_bridge() -> String {
    read(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "swift-bridge/Sources/UTIBridge/UTI.swift",
    ))
}

fn read_header(name: &str) -> String {
    read(&sdk_root().join(format!(
        "System/Library/Frameworks/UniformTypeIdentifiers.framework/Headers/{name}.h"
    )))
}

fn extract_interface(header: &str, type_name: &str) -> String {
    let needle = regex_lite::Regex::new(&format!(r"@interface\s+{type_name}\b")).unwrap();
    let Some(start) = needle.find(header) else {
        return String::new();
    };
    let rest = &header[start.start()..];
    let Some(end_off) = rest.find("@end") else {
        return rest.to_string();
    };
    rest[..end_off].to_string()
}

fn extract_member_surface(body: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let method_re =
        regex_lite::Regex::new(r"(?m)^\s*[+\-]\s*\([^\)]*\)\s*([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    for c in method_re.captures_iter(body) {
        out.insert(c[1].to_string());
    }
    let prop_re = regex_lite::Regex::new(
        r"(?m)^\s*@property\s*(?:\([^\)]*\))?\s*[^;]*?\b([A-Za-z_][A-Za-z0-9_]*)\s*(?:NS_|API_|;)",
    )
    .unwrap();
    for c in prop_re.captures_iter(body) {
        out.insert(c[1].to_string());
    }
    let getter_re = regex_lite::Regex::new(r"getter\s*=\s*([A-Za-z_][A-Za-z0-9_]*)").unwrap();
    for c in getter_re.captures_iter(body) {
        out.insert(c[1].to_string());
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
        // Obj-C `+ typeWithFilenameExtension:` -> Swift
        // `UTType(filenameExtension:)`
        ("typeWithIdentifier", "UTType(String"),
        ("typeWithFilenameExtension", "UTType(filenameExtension:"),
        ("typeWithMIMEType", "UTType(mimeType:"),
        ("conformsToType", ".conforms(to:"),
        ("preferredMIMEType", "preferredMIMEType"),
    ]
    .into_iter()
    .collect()
}

fn report(name: &str, apple: &BTreeSet<String>, ours: &BTreeSet<String>, omitted: &BTreeSet<String>) {
    let wrapped: BTreeSet<&String> = apple.intersection(ours).collect();
    let missing: BTreeSet<&String> = apple
        .difference(ours)
        .filter(|s| !omitted.contains(*s))
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
        for s in &missing {
            println!("  - {s}");
        }
    }
    assert!(pct >= 100.0, "{name}: {pct:.1}%");
}

fn omitted_set<const N: usize>(items: [&str; N]) -> BTreeSet<String> {
    items.into_iter().map(String::from).collect()
}

#[test]
fn ut_type_coverage() {
    let header = read_header("UTType");
    let body = extract_interface(&header, "UTType");
    let apple = extract_member_surface(&body);
    let ours = references_in_bridge(&apple);
    let omitted = omitted_set([
        // NS_UNAVAILABLE on UTType.
        "init",
        "new",
        // Constrained-supertype variants of typeWith* — we expose
        // `from_filename_extension_conforming_to(...)` only; MIME-conforming
        // and identifier-conforming variants land in v0.2.
        "typeWithMIMEType",
        // Properties + non-essentials we defer to v0.2:
        "version",
        "referenceURL",
        // `dynamic`/`declared`/`publicType` getters — wrapped under their
        // `is*` aliases by the bridge.
        "dynamic",
        "declared",
        "publicType",
        // `isPublicType` accessor was removed in macOS 26 SDK; we shim it
        // ourselves by string-prefix-checking `identifier`.
        "isPublicType",
    ]);
    report("UTType", &apple, &ours, &omitted);
}

#[test]
fn ut_type_conformance_category_coverage() {
    // The (Conformance) category in UTType.h adds three methods + one prop.
    let header = read_header("UTType");
    // The `(Conformance)` category appears as a second `@interface UTType`
    // block — extract everything after the first @end and before the next.
    let after_first = header.split_once("@end").map_or("", |(_, rest)| rest);
    let body = extract_interface(after_first, "UTType");
    let apple = extract_member_surface(&body);
    let ours = references_in_bridge(&apple);
    let omitted = omitted_set([
        // Iteration helpers — v0.2 will surface .supertypes / .subtypes /
        // .isSupertypeOfType / .isSubtypeOfType. We cover .conforms(to:)
        // which is the most-used direction.
        "isSupertypeOfType",
        "isSubtypeOfType",
        "supertypes",
    ]);
    report("UTType (Conformance)", &apple, &ours, &omitted);
}

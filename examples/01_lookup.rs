//! Look up UTI types by extension, MIME, and identifier.
//!
//! Run: `cargo run --example 01_lookup`

use uti::prelude::*;

#[allow(clippy::unnecessary_wraps)]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cases = [
        ("png", "image/png", "public.png"),
        ("jpg", "image/jpeg", "public.jpeg"),
        ("pdf", "application/pdf", "com.adobe.pdf"),
        ("json", "application/json", "public.json"),
        ("rs", "text/rust", "public.rust-source"),
        ("html", "text/html", "public.html"),
        ("mp3", "audio/mpeg", "public.mp3"),
        ("mov", "video/quicktime", "com.apple.quicktime-movie"),
    ];

    println!(
        "{:<6} {:<24} {:<32} description",
        "ext", "preferred MIME", "identifier"
    );
    let dashes = "-".repeat(110);
    println!("{dashes}");
    for (ext, _expected_mime, _expected_id) in &cases {
        match UTI::from_filename_extension(ext) {
            Ok(t) => {
                println!(
                    "{:<6} {:<24} {:<32} {}",
                    ext,
                    t.preferred_mime_type().unwrap_or_else(|| "—".into()),
                    t.identifier(),
                    t.localized_description().unwrap_or_else(|| "—".into()),
                );
            }
            Err(e) => println!("{ext:<6} ERROR: {e}"),
        }
    }

    println!("\n=== from_mime_type ===");
    for (_ext, mime, _) in &cases {
        match UTI::from_mime_type(mime) {
            Ok(t) => println!(
                "  {mime:<24} -> {} ({})",
                t.identifier(),
                t.preferred_filename_extension()
                    .unwrap_or_else(|| "—".into())
            ),
            Err(e) => println!("  {mime:<24} ERROR: {e}"),
        }
    }
    Ok(())
}

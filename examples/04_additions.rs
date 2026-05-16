//! Demonstrate `UTAdditions` path and URL helpers.
//!
//! Run: `cargo run --example 04_additions`

use uti::additions;
use uti::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let plain_text = UTI::well_known("plainText").unwrap();
    let base_path = "/Applications";
    let file_url = "file:///Applications/readme";

    let appended_component =
        additions::append_path_component_conforming_to(base_path, "notes", &plain_text)?;
    let appended_extension = additions::append_path_extension_for_type("readme", &plain_text)?;
    let appended_url_component =
        additions::append_url_path_component_conforming_to(file_url, "notes", &plain_text)?;
    let appended_url_extension =
        additions::append_url_path_extension_for_type(file_url, &plain_text)?;

    println!("path component: {}", appended_component.display());
    println!("path extension: {}", appended_extension.display());
    println!("url component: {appended_url_component}");
    println!("url extension: {appended_url_extension}");
    Ok(())
}

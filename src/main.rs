use anyhow::{Context, Ok, Result};
use element::create_parts;
use std::fs;

mod element;

fn read_file(path: &str) -> Result<String> {
    let file = fs::read(path).context("Failed to load file on path")?;
    let content = String::from_utf8(file)?;
    return Ok(content);
}

fn main() {
    let svg_file = read_file("icons/search-icon.svg");
    if svg_file.is_err() {
        return;
    }
    let svg_string = svg_file.unwrap();
    create_parts(&svg_string);
}

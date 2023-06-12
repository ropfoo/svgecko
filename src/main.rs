use anyhow::{Context, Ok, Result};
use element::create_node;
use std::{fs, ops::RangeFrom};

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
    let content = svg_string.replace("\n", "");
    let parts: Vec<&str> = content
        .split_terminator("<")
        .filter(|&row| row != "")
        .collect();

    create_node(&parts, RangeFrom { start: 1 });
}

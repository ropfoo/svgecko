use anyhow::{Context, Result};
use std::fs;

fn read_file(path: &str) -> Result<String> {
    let file = fs::read(path).context("Failed to load file on path")?;
    return String::from_utf8(file).context("Failed tp parse file");
}

fn main() {
    let svg_file = read_file("icons/search-icon.svg");
    match svg_file {
        Err(e) => println!("{}", e),
        Ok(result) => println!("{:?}", result),
    }
}

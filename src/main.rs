use std::fs;

fn main() {
    let svg_file = fs::read("icons/search-icon.svg");
    if svg_file.is_err() {
        panic!("Could not read file");
    }
    let content = svg_file.unwrap();
    let result = String::from_utf8(content);
    if result.is_err() {
        panic!("Failed parsing content")
    }
    println!("{:?}", result);
}

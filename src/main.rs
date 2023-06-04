use anyhow::{Context, Ok, Result};
use std::fs;

#[derive(Debug)]
struct Svg {
    width: i32,
    height: i32,
}

#[derive(Debug)]
enum Element {
    Svg(Svg),
}

impl Element {
    fn name(&self) -> &str {
        match self {
            Element::Svg(_) => "svg",
        }
    }
}

#[derive(Debug)]
struct Node {
    element: Element,
    children: Option<Vec<Node>>,
}

fn create_parts(s: &str) {
    let parts = s.split_terminator("<");
    let mut elements: Vec<Node> = vec![];
    parts.enumerate().for_each(|(idx, part)| {
        if idx == 1 {
            elements.push(Node {
                element: Element::Svg(Svg {
                    width: 10,
                    height: 10,
                }),
                children: None,
            })
        }

        if part.ends_with("/>") {}
    });
    println!("{:?}", elements)
}

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

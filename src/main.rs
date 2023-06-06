use anyhow::{Context, Ok, Result};
use std::{fs, process::id, str::FromStr};

fn split_attributes(s: &str, element_tag: &str) {
    let parts = s
        .replace(element_tag, "")
        .replace(">", "")
        .split_terminator("\" ")
        .filter(|&p| p.contains("="))
        .for_each(|p| {
            let thing = p.replace("\"", "");
            let res = thing.split_once("=");
            if res.is_some() {
                let (key, value) = res.unwrap();
                println!("key:{} ,value {}", key, value);
            }
        });
}

#[derive(Debug)]
struct Svg {
    width: i32,
    height: i32,
}

impl FromStr for Svg {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        split_attributes(s, "svg");
        // println!("{:?}", parts);
        return Ok(Svg {
            width: 0,
            height: 1,
        });
    }
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
            let attribute_name = part.split_whitespace().next().unwrap_or("");
            // TODO handle get by attribute
            Svg::from_str(part);
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

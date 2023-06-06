use anyhow::{Context, Ok, Result};
use std::{collections::HashMap, fs, process::id, str::FromStr};

fn split_attributes(s: &str, element_tag: &str) -> HashMap<String, String> {
    return s
        .replace(element_tag, "")
        .replace(">", "")
        .split_terminator("\" ")
        .filter(|&p| p.contains("="))
        .fold(HashMap::new(), |mut map, p| {
            let thing = p.replace("\"", "");
            let res = thing.split_once("=");
            if res.is_some() {
                let (key, value) = res.unwrap();
                map.insert(String::from(key), String::from(value));
            }
            return map;
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
        let parts = split_attributes(s, "svg");
        println!("{:?}", parts);
        let width: i32 = parts.get(" width").unwrap().parse().unwrap();
        let height: i32 = parts.get("height").unwrap().parse().unwrap();
        return Ok(Svg { width, height });
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
            let svg = Svg::from_str(part).unwrap();
            elements.push(Node {
                element: Element::Svg(svg),
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

use anyhow::{anyhow, Ok, Result};
use std::{collections::HashMap, str::FromStr};

use crate::element::svg::{Svg, SVG_TAG_NAME};

mod svg;

#[derive(Debug)]
struct Node {
    element: Element,
    children: Option<Vec<Node>>,
}

#[derive(Debug)]
pub enum Element {
    Svg(Result<Svg>),
}

impl FromStr for Element {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let attribute_name = s.split_whitespace().next().unwrap_or("");
        return match attribute_name {
            SVG_TAG_NAME => Ok(Element::Svg(Svg::from_str(s))),
            _ => Err(anyhow!("Could not create element from given string")),
        };
    }
}

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
pub fn create_parts(s: &str) {
    let parts = s.split_terminator("<");
    let mut nodes: Vec<Node> = vec![];
    parts.enumerate().for_each(|(idx, part)| {
        if idx == 1 {
            let element = Element::from_str(part);
            if element.is_ok() {
                nodes.push(Node {
                    element: element.unwrap(),
                    children: None,
                })
            }
        }

        if part.ends_with("/>") {}
    });
    println!("{:?}", nodes)
}

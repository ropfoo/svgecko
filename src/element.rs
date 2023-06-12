use anyhow::{anyhow, Ok, Result};
use std::collections::btree_map::Range;
use std::ops::RangeFrom;
use std::option;
use std::{collections::HashMap, str::FromStr};

use crate::element::circle::{Circle, CIRCLE_TAG_NAME};
use crate::element::svg::{Svg, SVG_TAG_NAME};
mod circle;
mod svg;

#[derive(Debug)]
struct Node {
    element: Element,
    children: Vec<Node>,
}

#[derive(Debug)]
pub enum Element {
    Svg(Result<Svg>),
    Circle(Result<Circle>),
}

impl FromStr for Element {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let attribute_name = s.split_whitespace().next().unwrap_or("");
        return match attribute_name {
            SVG_TAG_NAME => Ok(Element::Svg(Svg::from_str(s))),
            CIRCLE_TAG_NAME => Ok(Element::Circle(Circle::from_str(s))),
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
                let key = key.trim();
                map.insert(String::from(key), String::from(value));
            }
            return map;
        });
}

pub fn create_parts(s: &str) {
    let parts = s.split_terminator("<");
    let mut nodes: Vec<Node> = vec![];
    let mut it = 0;
    parts.enumerate().for_each(|(idx, part)| {
        let element = Element::from_str(part);
        if element.is_ok() {
            let node: Node = Node {
                element: element.unwrap(),
                children: vec![],
            };
            let len = part.len();
            let final_str = &part[len - 3..];
            println!("{}", final_str);

            if final_str.trim() == "/>" {
                nodes[it].children.push(node);
            } else {
                nodes.push(node);
                it += 1;
            }
        }
    });
    println!("{:?}", nodes)
}

pub fn create_node(parts: &Vec<&str>, range: RangeFrom<usize>) {
    let root = parts[0];
    let root_parts: Vec<&str> = root.split_whitespace().collect();
    let start = root_parts[0];

    println!("{:?} {}", root, start);

    let below_elements: Vec<&str> = parts.clone().drain(range).collect();
    for part in below_elements {
        let name = part.split_whitespace().next().unwrap_or("");
        println!("{:?}", name);
        if name != start {}
    }
}

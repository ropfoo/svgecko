use std::{collections::HashMap, str::FromStr};

use crate::element::svg::Svg;

mod svg;

#[derive(Debug)]
struct Node {
    element: Element,
    children: Option<Vec<Node>>,
}

#[derive(Debug)]
pub enum Element {
    Svg(Svg),
}

impl Element {
    fn name(&self) -> &str {
        match self {
            Element::Svg(_) => "svg",
        }
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

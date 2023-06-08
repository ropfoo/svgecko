use crate::element::split_attributes;
use anyhow::{Ok, Result};
use std::str::FromStr;

pub const CIRCLE_TAG_NAME: &str = "circle";

#[derive(Debug)]
pub struct Circle {
    fill: String,
    cx: f32,
    cy: f32,
    r: f32,
}

impl FromStr for Circle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = split_attributes(s, CIRCLE_TAG_NAME);
        println!("{:?}", parts);
        let cx = parts.get("cx").unwrap().parse().unwrap();
        let cy = parts.get("cy").unwrap().parse().unwrap();
        let r = parts.get("r").unwrap().parse().unwrap();
        // let fill = parts.get("fill").unwrap().parse().unwrap();
        return Ok(Circle {
            cx,
            cy,
            r,
            fill: String::from(""),
        });
    }
}

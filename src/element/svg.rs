use crate::element::split_attributes;
use anyhow::{Ok, Result};
use std::str::FromStr;

pub const SVG_TAG_NAME: &str = "svg";

#[derive(Debug)]
pub struct Svg {
    width: i32,
    height: i32,
}

impl FromStr for Svg {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts = split_attributes(s, "svg");
        println!("{:?}", parts);
        let width: i32 = parts.get("width").unwrap().parse().unwrap();
        let height: i32 = parts.get("height").unwrap().parse().unwrap();
        return Ok(Svg { width, height });
    }
}

#[macro_use]
extern crate lazy_static;

mod error;
mod lang;
mod parser;
mod substr;

use crate::parser::*;
use anyhow::Result;

const EXAMPLE_RS: &str = include_str!("../../../test/example.rs");

fn main() -> Result<()> {
    simple_logger::init().unwrap();
    let parser = BlockCommentParser::new("/*", "*/", true);
    let comments = parser.parse(EXAMPLE_RS)?;
    log::info!("{:?}", comments);
    Ok(())
}

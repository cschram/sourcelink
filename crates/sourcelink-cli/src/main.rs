mod error;
mod lang;
mod parser;
mod parsers;

use crate::lang::*;
use anyhow::Result;

const EXAMPLE_C: &str = include_str!("../../../test/example.c");

fn main() -> Result<()> {
    simple_logger::init().unwrap();
    let comments = Lang::from_filename("example.c")?.parse(EXAMPLE_C)?;
    log::info!("{:?}", comments);
    Ok(())
}

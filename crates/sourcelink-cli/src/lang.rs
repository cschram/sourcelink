use crate::{error::SourcelinkError, parser::*, parsers::*};
use anyhow::Result;
use std::{ffi::OsStr, path::Path};

#[derive(Debug)]
pub enum Lang {
    C,
    Go,
    JavaScript,
    Lua,
    Python,
    Rust,
}

impl Lang {
    pub fn from_filename(filename: &str) -> Result<Self> {
        let ext = Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
            .ok_or_else(|| SourcelinkError::UnknownLanguage(filename.to_owned()))?;
        match ext {
            "c" | "cpp" | "h" | "hpp" => Ok(Self::C),
            "go" => Ok(Self::Go),
            "js" | "jsx" | "ts" | "tsx" => Ok(Self::JavaScript),
            "lua" => Ok(Self::Lua),
            "py" => Ok(Self::Python),
            "rs" => Ok(Self::Rust),
            _ => Err(SourcelinkError::UnknownLanguage(filename.to_owned()).into()),
        }
    }

    pub fn parse<'source>(&self, content: &'source str) -> Result<Vec<Comment<'source>>> {
        let parser: Box<dyn Parser> = match self {
            Self::C => Box::new(CParser),
            Self::Go | Self::JavaScript => Box::new(GoParser),
            Self::Lua => Box::new(LuaParser),
            Self::Python => Box::new(PythonParser),
            Self::Rust => Box::new(RustParser),
            _ => unimplemented!(),
        };
        parser.parse(content)
    }
}

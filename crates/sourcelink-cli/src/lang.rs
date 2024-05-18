use crate::error::SourcelinkError;
use anyhow::Result;
use std::{collections::HashMap, ffi::OsStr, path::Path};

lazy_static! {
    static ref FILETYPES: HashMap<String, Lang> = {
        let mut ft = HashMap::new();

        // C/C++
        ft.insert("c".to_owned(), Lang::CFamily);
        ft.insert("cpp".to_owned(), Lang::CFamily);
        ft.insert("h".to_owned(), Lang::CFamily);
        ft.insert("hpp".to_owned(), Lang::CFamily);

        // Go
        ft.insert("go".to_owned(), Lang::Go);

        // JavaScript/TypeScript
        ft.insert("js".to_owned(), Lang::JavaScript);
        ft.insert("jsx".to_owned(), Lang::JavaScript);
        ft.insert("ts".to_owned(), Lang::JavaScript);
        ft.insert("tsx".to_owned(), Lang::JavaScript);

        // Python
        ft.insert("py".to_owned(), Lang::Python);

        // Rust
        ft.insert("rs".to_owned(), Lang::Rust);

        ft
    };
}

///
/// Programming Language selection
///
/// TODO: Many, many more languages.
///
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Lang {
    CFamily, // Includes C and C++
    Go,
    JavaScript, // Includes TypeScript as well as React extensions.
    Python,
    Rust,
}

pub struct LangParser {
    filetypes: HashMap<String, Lang>,
}

impl LangParser {
    pub fn new() -> Self {
        Self {
            filetypes: FILETYPES.clone(),
        }
    }

    pub fn with_filetypes(filetypes: HashMap<String, Lang>) -> Self {
        Self { filetypes }
    }

    fn get_lang(&self, filename: &str) -> Result<Lang> {
        Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
            .map(|ext| self.filetypes.get(ext))
            .flatten()
            .ok_or_else(|| SourcelinkError::UnknownLanguage(filename.to_owned()))
            .copied()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_language_detection() {
        let lp = LangParser::new();
        assert_eq!(lp.get_lang("test.c"), Ok(Lang::CFamily));
        assert_eq!(lp.get_lang("test.cpp"), Ok(Lang::CFamily));
        assert_eq!(lp.get_lang("test.js"), Ok(Lang::JavaScript));
        assert_eq!(lp.get_lang("test.ts"), Ok(Lang::JavaScript));
        assert_eq!(lp.get_lang("test.rs"), Ok(Lang::Rust));
        assert_eq!(
            lp.get_lang("test.bin"),
            Err(SourcelinkError::UnknownLanguage("test.bin".to_owned()))
        );
        assert_eq!(
            lp.get_lang("test"),
            Err(SourcelinkError::UnknownLanguage("test".to_owned()))
        );
    }
}

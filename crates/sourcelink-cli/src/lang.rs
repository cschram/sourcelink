use crate::{error::SourcelinkError, parser::*};
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
    /// .c, .h, .cpp, .hpp
    CFamily,
    /// .go
    Go,
    /// .js, .jsx, .ts, .tsx
    JavaScript,
    /// .py
    Python,
    /// .rs
    Rust,
}

impl Lang {
    pub fn parsers(&self) -> Vec<Box<dyn Parser>> {
        match self {
            Lang::CFamily | Lang::Go | Lang::JavaScript => vec![
                Box::new(SingleLineCommentParser::new("//")),
                Box::new(BlockCommentParser::new("/*", "*/", false)),
            ],
            Lang::Python => vec![
                Box::new(SingleLineCommentParser::new("#")),
                Box::new(BlockCommentParser::new(r#"""""#, r#"""""#, false)),
            ],
            Lang::Rust => vec![
                Box::new(SingleLineCommentParser::new("//")),
                Box::new(BlockCommentParser::new("/*", "*/", true)),
            ],
        }
    }
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

    #[allow(dead_code)]
    pub fn with_filetypes(filetypes: HashMap<String, Lang>) -> Self {
        Self { filetypes }
    }

    pub fn parse(&self, filename: &str, content: &str) -> Result<Vec<Comment>> {
        let lang = self.detect_lang(filename)?;
        let parsers = lang.parsers();
        let mut comments = vec![];
        for parser in parsers.iter() {
            comments.append(&mut parser.parse(content)?);
        }
        comments.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());
        Ok(comments)
    }

    fn detect_lang(&self, filename: &str) -> Result<Lang> {
        Path::new(filename)
            .extension()
            .and_then(OsStr::to_str)
            .map(|ext| self.filetypes.get(ext))
            .flatten()
            .ok_or_else(|| SourcelinkError::UnknownLanguage(filename.to_owned()).into())
            .copied()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_RS: &str = include_str!("../../../test/example.rs");

    #[test]
    fn lang_parser_detect_lang() {
        let lp = LangParser::new();
        assert!(matches!(lp.detect_lang("test.c"), Ok(Lang::CFamily)));
        assert!(matches!(lp.detect_lang("test.cpp"), Ok(Lang::CFamily)));
        assert!(matches!(lp.detect_lang("test.js"), Ok(Lang::JavaScript)));
        assert!(matches!(lp.detect_lang("test.ts"), Ok(Lang::JavaScript)));
        assert!(matches!(lp.detect_lang("test.rs"), Ok(Lang::Rust)));
        let test_bin = lp.detect_lang("test.bin");
        assert!(test_bin.is_err());
        assert_eq!(
            test_bin.unwrap_err().to_string().as_str(),
            "Unable to determine language of file test.bin",
        );
        let test = lp.detect_lang("test");
        assert!(test.is_err());
        assert_eq!(
            test.unwrap_err().to_string().as_str(),
            "Unable to determine language of file test",
        );
    }

    #[test]
    fn lang_parser_parse() {
        let lp = LangParser::new();
        let parsed = lp.parse("example.rs", EXAMPLE_RS);
        assert!(parsed.is_ok());
        let comments = parsed.unwrap();
        assert_eq!(comments.len(), 4);
        assert_eq!(
            comments[0],
            Comment {
                content: " comment one".to_owned(),
                start: 2,
                end: 14,
            }
        );
        assert_eq!(
            comments[1],
            Comment {
                content: " comment two ".to_owned(),
                start: 26,
                end: 39,
            }
        );
        assert_eq!(
            comments[2],
            Comment {
                content: " comment three".to_owned(),
                start: 70,
                end: 84,
            }
        );
        assert_eq!(
            comments[3],
            Comment {
                content: "\r\n/* comment four */\r\n".to_owned(),
                start: 91,
                end: 113,
            }
        );
    }
}

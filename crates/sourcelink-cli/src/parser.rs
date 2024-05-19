#![allow(dead_code)]
use crate::{error::SourcelinkError, substr::*};
use anyhow::Result;
use line_span::LineSpans;
use std::ops::Range;

#[derive(Clone, PartialEq, Debug)]
pub struct Comment {
    pub content: String,
    pub start: usize,
    pub end: usize,
}

impl Comment {
    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }
}

pub trait Parser {
    fn parse(&self, content: &str) -> Result<Vec<Comment>>;
}

/// Parser for single line comments.
pub struct SingleLineCommentParser {
    prefix: String,
}

impl SingleLineCommentParser {
    pub fn new(prefix: &str) -> Self {
        Self {
            prefix: prefix.to_owned(),
        }
    }
}

impl Parser for SingleLineCommentParser {
    fn parse(&self, content: &str) -> Result<Vec<Comment>> {
        let mut comments = vec![];
        for span in content.line_spans() {
            for i in span.range() {
                if (i + self.prefix.len()) <= span.end() {
                    if substr(content, i, self.prefix.len())? == self.prefix {
                        let start = i + self.prefix.len();
                        let len = span.end() - start;
                        comments.push(Comment {
                            content: substr(content, start, len)?,
                            start,
                            end: span.end(),
                        });
                    }
                }
            }
        }
        Ok(comments)
    }
}

/// Parser for block comments, optionally allowing for nested comments.
pub struct BlockCommentParser {
    prefix: String,
    postfix: String,
    allow_nested: bool,
}

impl BlockCommentParser {
    pub fn new(prefix: &str, postfix: &str, allow_nested: bool) -> Self {
        Self {
            prefix: prefix.to_owned(),
            postfix: postfix.to_owned(),
            allow_nested,
        }
    }
}

impl Parser for BlockCommentParser {
    fn parse(&self, content: &str) -> Result<Vec<Comment>> {
        let mut comments = vec![];
        let mut start = None;
        let mut nest_level = 0;
        for i in 0..content.len() {
            start = match start {
                Some(start) => {
                    // Detect if we've reached the end of the file
                    if (i + self.postfix.len()) > content.len() {
                        return Err(SourcelinkError::UnexpectedEOF.into());
                    }
                    if self.allow_nested && substr_eq(content, i, &self.prefix) {
                        // Detect a nested comment, if allowed
                        nest_level += 1;
                        Some(start)
                    } else if substr_eq(content, i, &self.postfix) {
                        // Decrement nest level if we're in a nested comment.
                        if nest_level > 0 {
                            nest_level -= 1;
                            Some(start)
                        } else {
                            comments.push(Comment {
                                content: substr(content, start, i - start)?,
                                start,
                                end: i,
                            });
                            None
                        }
                    } else {
                        Some(start)
                    }
                }
                None => {
                    if substr_eq(content, i, &self.prefix) {
                        Some(i + self.prefix.len())
                    } else {
                        None
                    }
                }
            }
        }
        Ok(comments)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use anyhow::Result;

    const EXAMPLE_RS: &str = include_str!("../../../test/example.rs");
    const EXAMPLE_C: &str = include_str!("../../../test/example.c");

    #[test]
    fn single_line_comment_parser() -> Result<()> {
        let parser = SingleLineCommentParser::new("//");
        let comments = parser.parse(EXAMPLE_RS)?;
        assert_eq!(comments.len(), 2);
        assert_eq!(comments[0].range(), 2..14);
        assert_eq!(comments[0].content, " comment one");
        assert_eq!(comments[1].range(), 70..84);
        assert_eq!(comments[1].content, " comment three");
        Ok(())
    }

    #[test]
    fn block_comment_parser_allow_nested() -> Result<()> {
        let parser = BlockCommentParser::new("/*", "*/", true);
        let comments = parser.parse(EXAMPLE_RS)?;
        assert_eq!(comments.len(), 2);
        assert_eq!(comments[0].range(), 26..39);
        assert_eq!(comments[0].content, " comment two ");
        assert_eq!(comments[1].range(), 91..113);
        assert_eq!(comments[1].content, "\r\n/* comment four */\r\n");
        Ok(())
    }

    #[test]
    fn block_comment_parser_deny_nested() -> Result<()> {
        let parser = BlockCommentParser::new("/*", "*/", false);
        let comments = parser.parse(EXAMPLE_C)?;
        assert_eq!(comments.len(), 2);
        assert_eq!(comments[0].range(), 28..41);
        assert_eq!(comments[0].content, " comment two ");
        assert_eq!(comments[1].range(), 85..103);
        assert_eq!(comments[1].content, "\r\n/* comment four ");
        Ok(())
    }
}

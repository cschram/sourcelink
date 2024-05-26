use crate::{error::*, parser::*};
use anyhow::Result;
use logos::Logos;

#[derive(Logos, Clone, PartialEq, Debug)]
#[logos(error = SourcelinkError)]
enum Token {
    #[token("\"")]
    DoubleQuote,
    #[token("'")]
    SingleQuote,
    #[token("`")]
    Backtick,
    #[token("//")]
    DoubleSlash,
    #[token("/*")]
    SlashStar,
    #[token("*/")]
    StarSlash,
    #[token("\n")]
    NewLine,
}

#[derive(Debug)]
enum ParseState {
    Empty,
    String(Token),
    SingleLineComment(usize),
    BlockComment(usize),
}

#[derive(Clone, Debug)]
pub struct GoParser;

impl<'source> Parser<'source> for GoParser {
    fn parse(&self, content: &'source str) -> Result<Vec<Comment<'source>>> {
        let mut comments = vec![];
        let mut state = ParseState::Empty;
        let mut lex = Token::lexer(content);
        while let Some(result) = lex.next() {
            if let Ok(current_token) = result {
                state = match current_token {
                    Token::DoubleQuote | Token::SingleQuote | Token::Backtick => match &state {
                        ParseState::Empty => ParseState::String(current_token.clone()),
                        ParseState::String(token) => {
                            if current_token == *token {
                                ParseState::Empty
                            } else {
                                state
                            }
                        }
                        _ => state,
                    },
                    Token::DoubleSlash => {
                        if matches!(state, ParseState::Empty) {
                            ParseState::SingleLineComment(lex.span().end)
                        } else {
                            state
                        }
                    }
                    Token::SlashStar => {
                        if matches!(state, ParseState::Empty) {
                            ParseState::BlockComment(lex.span().end)
                        } else {
                            state
                        }
                    }
                    Token::StarSlash => {
                        if let ParseState::BlockComment(start) = state {
                            let end = lex.span().start;
                            comments.push(Comment::new(substr(content, start, end)?, start, end));
                            ParseState::Empty
                        } else {
                            state
                        }
                    }
                    Token::NewLine => {
                        if let ParseState::SingleLineComment(start) = state {
                            let end = lex.span().start;
                            comments.push(Comment::new(substr(content, start, end)?, start, end));
                            ParseState::Empty
                        } else {
                            state
                        }
                    }
                };
            }
        }
        match state {
            ParseState::Empty => Ok(comments),
            _ => Err(SourcelinkError::ParseError.into()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_GO: &str = include_str!("../../../../test/example.go");

    #[test]
    fn parse() {
        let parser = GoParser;
        let result = parser.parse(EXAMPLE_GO);
        assert!(result.is_ok());
        let comments = result.unwrap();
        assert_eq!(comments.len(), 4);
        assert_eq!(
            comments[0].content(),
            " https://github.com/cschram/sourcelink\r"
        );
        assert_eq!(comments[1].content(), " lorem ipsum ");
        assert_eq!(comments[2].content(), " https://www.google.com\r");
        assert_eq!(comments[3].content(), "\r\n/* lorem ipsum ");
    }
}

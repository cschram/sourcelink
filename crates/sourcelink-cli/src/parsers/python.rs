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
    #[token("#")]
    Hash,
    #[token("\"\"\"")]
    TripleQuote,
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
pub struct PythonParser;

impl<'source> Parser<'source> for PythonParser {
    fn parse(&self, content: &'source str) -> Result<Vec<Comment<'source>>> {
        let mut comments = vec![];
        let mut state = ParseState::Empty;
        let mut lex = Token::lexer(content);
        while let Some(result) = lex.next() {
            if let Ok(current_token) = result {
                state = match current_token {
                    Token::DoubleQuote | Token::SingleQuote => match &state {
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
                    Token::Hash => {
                        if matches!(state, ParseState::Empty) {
                            ParseState::SingleLineComment(lex.span().end)
                        } else {
                            state
                        }
                    }
                    Token::TripleQuote => match state {
                        ParseState::Empty => ParseState::BlockComment(lex.span().end),
                        ParseState::BlockComment(start) => {
                            let end = lex.span().start;
                            comments.push(Comment::new(substr(content, start, end)?, start, end));
                            ParseState::Empty
                        }
                        _ => state,
                    },
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

    const EXAMPLE_PY: &str = include_str!("../../../../test/example.py");

    #[test]
    fn parse() {
        let parser = PythonParser;
        let result = parser.parse(EXAMPLE_PY);
        assert!(result.is_ok());
        let comments = result.unwrap();
        assert_eq!(comments.len(), 3);
        assert_eq!(
            comments[0].content(),
            " https://github.com/cschram/sourcelink\r"
        );
        assert_eq!(
            comments[1].content(),
            "\r\n    # https://www.google.com\r\n    "
        );
        assert_eq!(comments[2].content(), " lorem ipsum\r");
    }
}

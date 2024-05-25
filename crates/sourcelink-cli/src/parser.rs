use logos::Logos;
use anyhow::Result; 

#[derive(Logos, Clone, PartialEq, Debug)]
struct Token {
    // Comment delimeters
    #[token("//")]
    DoubleSlash,
    #[token("/*")]
    SlashStar,
    #[token("*/")]
    StarSlash,
    #[token("#")]
    Hash,
    #[token("--")]
    DashDash,
    #[token("\"\"\"")]
    TripleQuote,

    // String delimeters
    #[token("\"")]
    Quote,
    #[token("'")]
    SingleQuote,
    #[token("`")]
    Backtick,
}

#[derive(Debug)]
enum ParserState {
    None,
    String {
        token: Token,
        start: usize,
    },
    Comment {
        token: Token,
        start: usize,
    },
    BlockComment {
        token: Token,
        start: usize,
    },
}

#[derive(Clone, Debug)]
struct Comment {
    content: String,
    start: usize,
    end: usize,
}

pub trait Parser {
    fn parse(&self, content: &str) -> Result<Vec<Comment>>;
}


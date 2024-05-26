use thiserror::Error;

#[derive(Default, Error, PartialEq, Clone, Debug)]
pub enum SourcelinkError {
    #[default]
    #[error("Error parsing source")]
    ParseError,
    #[error("Unable to determine language of file {0}")]
    UnknownLanguage(String),
    #[error("Value {0} out of range {1}..{2}")]
    OutOfRange(usize, usize, usize),
    #[error("Unexpectedly reached end of content")]
    UnexpectedEOF,
}

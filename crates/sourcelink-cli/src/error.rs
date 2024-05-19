use thiserror::Error;

#[derive(Error, PartialEq, Clone, Debug)]
pub enum SourcelinkError {
    #[error("Unable to determine language of file {0}")]
    UnknownLanguage(String),
    #[error("Substring out of range")]
    SubstrRange,
    #[error("Unexpectedly reached end of content")]
    UnexpectedEOF,
}

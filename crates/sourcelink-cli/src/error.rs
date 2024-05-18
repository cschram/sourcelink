use thiserror::Error;

#[derive(Error, Clone, Debug)]
pub enum SourcelinkError {
    #[error("Unable to determine language of file {0}")]
    UnknownLanguage(String),
}

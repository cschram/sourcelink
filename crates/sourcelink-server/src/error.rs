use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SourcelinkError {
    #[error("{0}")]
    SQLXError(#[from] sqlx::Error),
    #[error("Unauthorized")]
    Unauthorized,
}

impl SourcelinkError {
    fn kind(&self) -> String {
        match self {
            Self::SQLXError(_) => "SQLXError",
            Self::Unauthorized => "Unauthorized",
        }
        .to_owned()
    }
}

impl IntoResponse for SourcelinkError {
    fn into_response(self) -> Response {
        log::error!(target: "api",
            error_kind = self.kind();
            "{}",
            self.to_string(),
        );
        match self {
            Self::SQLXError(err) => match err {
                sqlx::Error::RowNotFound => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
        }
        .into_response()
    }
}

pub type Result<T> = std::result::Result<T, SourcelinkError>;

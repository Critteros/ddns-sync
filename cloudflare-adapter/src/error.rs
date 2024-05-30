use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    NetworkError(#[from] reqwest::Error),

    #[error("Invalid response")]
    InvalidResponse,

    #[error(transparent)]
    ParseError(#[from] serde_json::Error),
}

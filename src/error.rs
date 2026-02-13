use thiserror::Error;

#[derive(Error, Debug)]
pub enum SrpError {
    #[error("Internal error: {0}")]
    Internal(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Analysis error: {0}")]
    Analysis(String),
}

pub type SrpResult<T> = Result<T, SrpError>;

use thiserror::Error;

/// Error type for Ncm2Mp3
#[allow(missing_docs)]
#[derive(Error, Debug)]
pub enum NcmError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid data: {0}")]
    Invalid(String),
    #[error("Invalid music tag")]
    InvalidTag(#[from] id3::Error),
}

/// type alias for `Result`
pub type Result<T> = core::result::Result<T, NcmError>;

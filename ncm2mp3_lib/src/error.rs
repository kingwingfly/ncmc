use thiserror::Error;

#[derive(Error, Debug)]
pub enum NcmError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid data")]
    Invalid,
}

pub type Result<T> = core::result::Result<T, NcmError>;

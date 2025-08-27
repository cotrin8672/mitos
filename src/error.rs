use thiserror::Error;

#[derive(Error, Debug)]
pub enum MitosError {
    #[error("Git repository eror: {0}")]
    GitError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type MitosResult<T> = Result<T, MitosError>;

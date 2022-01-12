use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Error opening file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("File not found.")]
    FileNotFound,
}

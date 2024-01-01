use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    GenericError(String),

    #[error("Error opening file: {0}")]
    IoError(#[from] std::io::Error),

    #[error("File not found.")]
    FileNotFound,

    #[error("Error parsing line: {message}")]
    ParseIntError {
        message: String,
        #[source]
        source:  ParseIntError,
    },
    #[error("Error getting user input")]
    UserInputError {
        #[from]
        source: dialoguer::Error,
    },
}

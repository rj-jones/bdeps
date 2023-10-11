use std::{error::Error, fmt};

use crate::command::CommandError;

// Note: This module's purpose is to maintain structured logging
// while still being able to return a string error in main.rs.

#[derive(Debug)]
enum ErrorWrapper {
    CommandError(CommandError),
    // OtherError,
    // ... other error variants
}

impl fmt::Display for ErrorWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorWrapper::CommandError(e) => write!(f, "Command error: {}", e),
            // ErrorWrapper::OtherError => write!(f, "An other error occurred"),
            // ... other error variants
        }
    }
}

impl Error for ErrorWrapper {}

// ...implement From trait for each error type you want to convert...

impl From<CommandError> for ErrorWrapper {
    fn from(error: CommandError) -> Self {
        ErrorWrapper::CommandError(error)
    }
}

// ... other From implementations ...

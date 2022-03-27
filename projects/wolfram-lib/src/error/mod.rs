use std::{
    error::Error,
    fmt::{Debug, Display, Formatter},
    panic::Location,
};

/// Result type for error handling.
pub type Result<T> = std::result::Result<T, WolframError>;

/// All Errors that can occur in the Wolfram-Lib
#[derive(Debug)]
pub enum WolframError {
    /// Error while parsing JSON
    NullException(String),
    /// Error while parsing a string
    SyntaxError(String),
    /// Error while parsing a string
    IOError(std::io::Error),
}

impl Display for WolframError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for WolframError {}

impl From<()> for WolframError {
    fn from(_: ()) -> Self {
        let loc = Location::caller();

        WolframError::NullException(format!("{:?}", loc))
    }
}

impl From<std::io::Error> for WolframError {
    fn from(e: std::io::Error) -> Self {
        WolframError::IOError(e)
    }
}

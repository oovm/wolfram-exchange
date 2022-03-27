/// Result type for error handling.
pub type Result<T> = std::result::Result<T, WolframError>;

/// All Errors that can occur in the Wolfram-Lib
pub enum WolframError {
    /// Error while parsing a string
    SyntaxError(String),
}

//! Error types for sea-lex

use thiserror::Error;

/// Error that can occur during lexing
#[derive(Debug, Error, Clone, PartialEq, Eq)]
#[error("Lex error at position {position}: {message}")]
pub struct LexError {
    /// The position in the input where the error occurred
    pub position: usize,
    /// A description of the error
    pub message: String,
}

impl LexError {
    /// Create a new lexing error
    pub fn new(position: usize, message: impl Into<String>) -> Self {
        Self {
            position,
            message: message.into(),
        }
    }
}
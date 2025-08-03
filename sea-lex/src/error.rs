//! Error types for sea-lex

use regex::Error as RegexError;
use thiserror::Error;

/// Error that can occur during lexing
#[derive(Debug, Error)]
pub enum LexError {
    /// An unexpected character was encountered during lexing
    #[error("Unexpected character at position {position}: '{character}'")]
    UnexpectedChar { 
        /// The position in the input where the error occurred
        position: usize, 
        /// The unexpected character
        character: char 
    },
    /// An invalid regular expression pattern was provided
    #[error("Invalid regex pattern '{pattern}': '{error}'")]
    InvalidRegex { 
        /// The invalid regex pattern
        pattern: String, 
        /// The underlying regex error
        error: RegexError 
    },
    /// An error occurred while parsing a token field value
    #[error("Error parsing token at position {position}: {error}")]
    TokenParseError { 
        /// The position in the input where the error occurred
        position: usize, 
        /// The underlying parsing error
        error: Box<dyn std::error::Error> 
    },
}

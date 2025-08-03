//! Token types for sea-lex

/// A token with position information
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenInfo<T> {
    /// The token variant
    pub kind: T,
    /// The text that was matched
    pub text: String,
    /// The start position in the input
    pub start: usize,
    /// The end position in the input (exclusive)
    pub end: usize,
}

impl<T> TokenInfo<T> {
    /// Create a new token with position information
    pub fn new(kind: T, text: impl Into<String>, start: usize, end: usize) -> Self {
        Self {
            kind,
            text: text.into(),
            start,
            end,
        }
    }
}
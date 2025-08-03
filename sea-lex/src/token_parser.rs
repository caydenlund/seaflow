//! Token parser trait and implementations

use crate::LexError;

/// Trait for parsing token field values from matched text
pub trait TokenParser<T> {
    /// Parse the matched text into a value of type T
    ///
    /// # Errors
    ///
    /// Returns a `LexError::TokenParseError` if parsing fails
    fn parse(&self, input: &str, position: usize) -> Result<T, LexError>;
}

/// Helper trait to distinguish between different return types
pub trait IntoTokenResult<T> {
    /// Convert the result into a `Result<T, LexError>`
    fn into_token_result(self, position: usize) -> Result<T, LexError>;
}

/// Implement [`IntoTokenResult`] for T (success case)
impl<T> IntoTokenResult<T> for T {
    fn into_token_result(self, _position: usize) -> Result<T, LexError> {
        Ok(self)
    }
}

/// Implement [`IntoTokenResult`] for Result<T, E>
impl<T, E> IntoTokenResult<T> for Result<T, E>
where
    E: std::error::Error + 'static,
{
    fn into_token_result(self, position: usize) -> Result<T, LexError> {
        self.map_err(|e| LexError::TokenParseError {
            position,
            error: Box::new(e),
        })
    }
}

/// Implement [`TokenParser`] for functions that return something convertible to `Result<T, LexError>`
impl<F, T, R> TokenParser<T> for F
where
    F: Fn(&str) -> R,
    R: IntoTokenResult<T>,
{
    fn parse(&self, input: &str, position: usize) -> Result<T, LexError> {
        self(input).into_token_result(position)
    }
}


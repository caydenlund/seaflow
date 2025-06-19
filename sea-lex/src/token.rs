//! sea-lex/token - Token representation for lexical analysis.
//!
//! This module provides the [`Token`] struct which represents a single token
//! extracted from source code during lexing, containing the following data:
//! - Its type (implementing [`TokenType`])
//! - The original source text it was created from
//! - Its position in the source stream
//!
//! # Example
//! ```
//! # use sea_lex::*;
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! enum MyTokenType {
//!     Number,
//!     Identifier,
//! }
//!
//! impl TokenType for MyTokenType {
//!     fn matchers() -> Vec<(TokenCreator<Self>, TokenMatcher)> { Vec::new() }
//! }
//!
//! let token = Token::new(MyTokenType::Number, "42", 0);
//! assert_eq!(token.typ, MyTokenType::Number);
//! assert_eq!(token.contents, "42");
//! assert_eq!(token.position, 0);
//! ```

use crate::TokenType;

/// A lexical token extracted from source code.
///
/// Contains both semantic information (the token type) and the original
/// source representation along with its position.
///
/// # Type Parameters
/// - `T`: The token type which must implement [`TokenType`]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<T>
where
    T: TokenType,
{
    /// The semantic type of the token.
    pub typ: T,
    /// The original source text that produced this token.
    pub contents: String,
    /// The byte position where this token starts in the source.
    pub position: usize,
}

impl<T> Token<T>
where
    T: TokenType,
{
    /// Creates a new token with the given properties.
    ///
    /// # Arguments
    /// - `typ` - The semantic type of the token
    /// - `contents` - The source text this token represents
    /// - `position` - The byte offset where this token starts in the source
    pub fn new<S: Into<String>>(typ: T, contents: S, position: usize) -> Self {
        Self {
            typ,
            contents: contents.into(),
            position,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Token, TokenCreator, TokenMatcher, TokenType};

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum TestToken {
        Number,
        Identifier,
        Plus,
        Minus,
    }

    impl TokenType for TestToken {
        fn matchers() -> Vec<(TokenCreator<Self>, TokenMatcher)> {
            Vec::new() // Not needed for token tests
        }
    }

    #[test]
    fn test_token_creation() {
        let token = Token::new(TestToken::Number, "42", 0);
        assert_eq!(token.typ, TestToken::Number);
        assert_eq!(token.contents, "42");
        assert_eq!(token.position, 0);
    }

    #[test]
    fn test_token_with_string_contents() {
        let token = Token::new(TestToken::Identifier, "foo_bar", 10);
        assert_eq!(token.typ, TestToken::Identifier);
        assert_eq!(token.contents, "foo_bar");
        assert_eq!(token.position, 10);
    }

    #[test]
    fn test_token_with_different_positions() {
        let token1 = Token::new(TestToken::Plus, "+", 5);
        let token2 = Token::new(TestToken::Plus, "+", 10);

        assert_eq!(token1.typ, token2.typ);
        assert_eq!(token1.contents, token2.contents);
        assert_ne!(token1.position, token2.position);
    }

    #[test]
    fn test_token_equality() {
        let token1 = Token::new(TestToken::Minus, "-", 3);
        let token2 = Token::new(TestToken::Minus, "-", 3);
        let token3 = Token::new(TestToken::Minus, "--", 3);
        let token4 = Token::new(TestToken::Plus, "-", 3);

        assert_eq!(token1, token2);
        assert_ne!(token1, token3);
        assert_ne!(token1, token4);
    }

    #[test]
    fn test_token_with_string_conversion() {
        let token = Token::new(TestToken::Identifier, String::from("variable"), 7);
        assert_eq!(token.contents, "variable");
    }

    #[test]
    fn test_token_with_empty_contents() {
        let token = Token::new(TestToken::Number, "", 0);
        assert_eq!(token.contents, "");
    }

    #[test]
    fn test_token_with_large_position() {
        let token = Token::new(TestToken::Plus, "+", usize::MAX);
        assert_eq!(token.position, usize::MAX);
    }

    #[test]
    fn test_token_clone() {
        let token1 = Token::new(TestToken::Identifier, "xyz", 15);
        let token2 = token1.clone();
        assert_eq!(token1, token2);
    }

    #[test]
    fn test_token_debug_format() {
        let token = Token::new(TestToken::Number, "123", 5);
        let debug_output = format!("{:?}", token);
        assert!(debug_output.contains("Number"));
        assert!(debug_output.contains("123"));
        assert!(debug_output.contains("5"));
    }
}

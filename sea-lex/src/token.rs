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

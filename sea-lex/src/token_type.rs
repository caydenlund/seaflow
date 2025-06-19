//! sea-lex/token-type - Provides the [`TokenType`] trait for creating a [`Lexer`].
//!
//! The [`TokenType`] trait provides the following:
//! - A standard interface for defining programming language tokens
//! - The ability to generate a [`Lexer`] from token definitions
//! - A way to specify token matching rules with priority ordering

use crate::{Lexer, TokenCreator, TokenMatcher};

/// Provides a method to create a new [`Lexer`] for a language definition.
///
/// This trait serves as the main interface for defining lexical analyzers.
/// Types implementing this trait represent the different token types of a language
/// and define how those tokens are recognized from source code.
///
/// # Examples
/// Implementing a simple language with two token types:
/// ```
/// # use sea_lex::*;
/// use regex::Regex;
///
/// #[derive(Clone)]
/// pub enum EquationToken {
///     Integer(i64),
///     Identifier(String),
///     LParen,
///     RParen,
///     Asterisk,
///     Slash,
///     Plus,
///     Minus,
/// }
/// impl TokenType for EquationToken {
///     fn matchers() -> Vec<(TokenCreator<Self>, TokenMatcher)> {
///         vec![
///             (
///                 TokenCreator::Fn(Box::new(|c| Self::Integer(c.parse().unwrap()))),
///                 Regex::new(r"^\d+").unwrap().into(),
///             ),
///             (
///                 TokenCreator::Fn(Box::new(|c| Self::Identifier(c.to_string()))),
///                 Regex::new(r"^[a-zA-Z_]+").unwrap().into(),
///             ),
///             (Self::LParen.into(), "(".into()),
///             (Self::RParen.into(), ")".into()),
///             (Self::Asterisk.into(), "*".into()),
///             (Self::Slash.into(), "/".into()),
///             (Self::Plus.into(), "+".into()),
///             (Self::Minus.into(), "-".into()),
///         ]
///     }
/// }
/// ```
pub trait TokenType: Sized + Clone {
    /// Generates a new [`Lexer`] for this [`TokenType`].
    ///
    /// This provides a convenient default implementation that creates a lexer
    /// using the matchers defined in [`matchers()`](TokenType::matchers).
    ///
    /// # Returns
    /// A new [`Lexer`] for this [`TokenType`].
    #[must_use]
    fn lexer() -> Lexer<Self> {
        Lexer {
            matchers: Self::matchers(),
        }
    }

    /// Returns a set of [`Matcher<Self>`], ordered by priority.
    ///
    /// Implementers of the [`TokenType`] trait must define an ordered list of
    /// pattern matchers with corresponding token creators.
    /// Earlier matchers have higher priority.
    ///
    /// # Returns
    /// This language's set of [`Matcher<Self>`], ordered by priority.
    fn matchers() -> Vec<(TokenCreator<Self>, TokenMatcher)>;
}

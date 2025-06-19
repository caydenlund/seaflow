//! sea-lex/lexer - Lexer module for tokenizing source code.
//!
//! This module provides a [`Lexer`] struct that can be used to convert source code
//! into a sequence of tokens. The lexer uses a set of matchers to identify token
//! patterns in the source text.
//!
//! # Example
//! ```
//! # use sea_lex::*;
//! use regex::Regex;
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! pub enum EquationToken {
//!     Integer(i64),
//!     Asterisk,
//!     Slash,
//!     Plus,
//!     Minus,
//! }
//! impl TokenType for EquationToken {
//!     fn matchers() -> Vec<(TokenCreator<Self>, TokenMatcher)> {
//!         vec![
//!             (
//!                 TokenCreator::Fn(Box::new(|c| Self::Integer(c.parse().unwrap()))).into(),
//!                 Regex::new(r"^\d+").unwrap().into(),
//!             ),
//!             (Self::Asterisk.into(), "*".into()),
//!             (Self::Slash.into(), "/".into()),
//!             (Self::Plus.into(), "+".into()),
//!             (Self::Minus.into(), "-".into()),
//!             (TokenCreator::None, " ".into()), // skip spaces
//!         ]
//!     }
//! }
//!
//! let lexer = EquationToken::lexer();
//! let tokens = lexer.lex("1 + 23 - 45 * 6 / 789");
//! let expected = vec![
//!     EquationToken::Integer(1),
//!     EquationToken::Plus,
//!     EquationToken::Integer(23),
//!     EquationToken::Minus,
//!     EquationToken::Integer(45),
//!     EquationToken::Asterisk,
//!     EquationToken::Integer(6),
//!     EquationToken::Slash,
//!     EquationToken::Integer(789),
//! ];
//! assert_eq!(tokens, Ok(expected));
//! ```

use crate::LexerError;
use crate::TokenCreator;
use crate::TokenMatcher;
use crate::TokenType;

/// A lexer that converts source text into tokens of type `T`.
///
/// The lexer maintains a collection of [`Matcher`] instances that are used
/// to identify and extract tokens from the input string.
pub struct Lexer<T>
where
    T: TokenType,
{
    /// Collection of token matchers and creators used to identify tokens.
    ///
    /// Matchers are applied in order, with the first successful match
    /// determining the token to construct.
    pub(crate) matchers: Vec<(TokenCreator<T>, TokenMatcher)>,
}

impl<T> Lexer<T>
where
    T: TokenType,
{
    /// Converts source text into a sequence of tokens.
    ///
    /// # Arguments
    /// * `source` - The input string to be tokenized.
    ///
    /// # Returns
    /// - `Ok(Vec<T>)` with the parsed tokens on success.
    /// - `Err(LexerError)` if lexing fails.
    ///
    /// # Errors
    /// Returns an error if unmatched text (an illegal character sequence) is encountered.
    ///
    /// # Behavior
    /// - Processes the input string from start to end.
    /// - For each position, tries matchers in order until a match is found.
    /// - Stops when no more tokens can be matched.
    pub fn lex(&self, source: &str) -> Result<Vec<T>, LexerError> {
        let mut tokens = Vec::new();
        let mut remaining = source;
        let mut position = 0;

        'outer: while !remaining.is_empty() {
            for (creator, matcher) in &self.matchers {
                if let Some(len) = matcher.matches(remaining) {
                    let matched_text = &remaining[..len];
                    remaining = &remaining[len..];
                    position += len;

                    if let Some(token) = creator.create(matched_text) {
                        tokens.push(token);
                    }
                    continue 'outer;
                }
            }

            return Err(LexerError {
                position,
                unmatched: remaining.chars().next().unwrap_or_default(),
            });
        }

        Ok(tokens)
    }
}

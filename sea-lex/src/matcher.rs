//! sea-lex/matcher - Pattern matching and token creation system.
//!
//! This module provides functionality for matching text patterns and creating tokens
//! from matched content. It combines [`MatcherPattern`] for pattern matching with
//! [`TokenCreator`] for token generation.
//!
//! # Components
//! - [`MatcherPattern`]: Enum for different pattern matching strategies (literal/regex)
//! - [`Matcher`]: Combines a pattern matcher with a token creator
//!
//! # Examples
//! ```
//! # use sea_lex::*;
//! use regex::Regex;
//!
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! enum Token {
//!     WithContents(i32),
//!     Static,
//! }
//! impl TokenType for Token {
//!     fn matchers() -> Vec<Matcher<Self>> { Vec::new() }
//! }
//!
//! // Literal pattern with cloned token
//! let matcher = Matcher::from((Token::Static, "prefix_"));
//! assert_eq!(matcher.pattern.matches("prefix_value"), Some(7));
//!
//! // Regex pattern with transformation
//! let regex = Regex::new(r"^\d+").unwrap();
//! let creator = TokenCreator::Fn(Box::new(|s| Token::WithContents(s.parse().unwrap())));
//! let matcher = Matcher::from((creator, regex));
//! ```
//!
//! # Pattern Types
//! - Literal strings: Exact prefix matching
//! - Regular expressions: Flexible pattern matching via `regex` crate

use crate::{TokenCreator, TokenType};
use regex::Regex;

/// Matches a pattern in a string.
pub enum MatcherPattern {
    /// Matches exact string prefixes.
    Literal(String),
    /// Matches using regular expressions.
    Regex(Regex),
}

impl MatcherPattern {
    /// Checks if the pattern matches at the given position of the input text.
    ///
    /// # Arguments
    /// * `text` - The input text to match against.
    ///
    /// # Returns
    /// - `Some(len)` with the match length if the pattern matches.
    /// - `None` if the pattern doesn't match.
    #[must_use]
    pub fn matches(&self, input: &str) -> Option<usize> {
        match self {
            Self::Literal(s) => input.starts_with(s).then_some(s.len()),
            Self::Regex(r) => r.find(input).map(|m| m.end()),
        }
    }
}

impl From<&str> for MatcherPattern {
    /// Converts a string slice into a literal pattern matcher.
    ///
    /// # Arguments
    /// - `s`: A literal string to match.
    ///
    /// # Returns
    /// A new [`MatcherPattern`] for the given string slice.
    fn from(s: &str) -> Self {
        Self::Literal(s.to_string())
    }
}

impl From<Regex> for MatcherPattern {
    /// Converts a `Regex` into a regex pattern matcher.
    ///
    /// # Arguments
    /// - `regex`: A compiled regular expression.
    ///
    /// # Returns
    /// A new [`MatcherPattern`] for the given regular expression.
    fn from(regex: Regex) -> Self {
        Self::Regex(regex)
    }
}

/// Combines a pattern matcher with a token creator.
pub struct Matcher<T>
where
    T: TokenType,
{
    /// The token creator.
    pub creator: TokenCreator<T>,
    /// The pattern to match.
    pub pattern: MatcherPattern,
}

impl<T> Matcher<T>
where
    T: TokenType,
{
    /// Applies the pattern matcher and token creator on the given string.
    ///
    /// # Arguments
    /// - `input` - The input string to test for a pattern match.
    ///
    /// # Returns
    /// - `Some(self.creator.create(matched_substr), len)` if the pattern is a match.
    /// - `None` otherwise.
    pub fn apply(&self, input: &str) -> Option<(Option<T>, usize)> {
        self.pattern.matches(input).map(|len| {
            (
                self.creator
                    .create(&input.chars().take(len).collect::<String>()),
                len,
            )
        })
    }
}

impl<T, C> From<(C, Regex)> for Matcher<T>
where
    C: Into<TokenCreator<T>>,
    T: TokenType,
{
    /// Creates a [`Matcher`] from a token creator and regex pattern.
    ///
    /// # Arguments
    /// * `(creator, regex)` - A tuple with two fields:
    ///   - Any type convertible to [`TokenCreator<T>`]
    ///   - A compiled regular expression
    ///
    /// # Returns
    /// A newly-constructed [`Matcher`].
    fn from((creator, regex): (C, Regex)) -> Self {
        Self {
            creator: creator.into(),
            pattern: MatcherPattern::Regex(regex),
        }
    }
}

impl<T, C> From<(C, &str)> for Matcher<T>
where
    C: Into<TokenCreator<T>>,
    T: TokenType,
{
    /// Creates a [`Matcher`] from a token creator and literal string pattern.
    ///
    /// # Arguments
    /// * `(creator, literal)` - A tuple with two fields:
    ///   - Any type convertible to [`TokenCreator<T>`]
    ///   - A string literal pattern
    ///
    /// # Returns
    /// A newly-constructed [`Matcher`].
    fn from((creator, literal): (C, &str)) -> Self {
        Self {
            creator: creator.into(),
            pattern: MatcherPattern::Literal(literal.to_string()),
        }
    }
}

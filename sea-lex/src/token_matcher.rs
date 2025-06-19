//! sea-lex/token-matcher - A module for matching tokens in input strings.
//!
//! This module provides two implementations of the [`TokenMatcher`] trait:
//! - [`LiteralMatcher`] for exact string matches.
//! - [`RegexMatcher`] for pattern-based matches using regular expressions.
//!
//! # Examples
//! ```
//! # use sea_lex::*;
//! # use regex::Regex;
//! # || -> Result<(), regex::Error> {
//! let lit_matcher = TokenMatcher::Literal("fn".into());
//! assert_eq!(lit_matcher.matches("fn fn"), Some(2));
//! assert_eq!(lit_matcher.matches("function fn"), None);
//!
//! let regex_matcher = TokenMatcher::Regex(Regex::new(r"\d+")?);
//! assert_eq!(regex_matcher.matches("123abc"), Some(3));
//! assert_eq!(regex_matcher.matches("abc123"), None);
//!
//! let fn_matcher = TokenMatcher::Fn(Box::new(|_| Some(1)));
//! assert_eq!(fn_matcher.matches(""), Some(1));
//! assert_eq!(fn_matcher.matches("foo"), Some(1));
//! assert_eq!(fn_matcher.matches("bar"), Some(1));
//! # Ok(())
//! # };
//! ```

use regex::Regex;

/// A matcher function for lexing.
///
/// If a match at the beginning of the input string is detected, emit `Some(match_len)`.
pub type MatcherFn = Box<dyn Fn(&str) -> Option<usize>>;

/// An enum representing different ways to match tokens at the beginning of input strings.
pub enum TokenMatcher {
    /// Matches an exact string literal.
    Literal(String),
    /// Matches using a regular expression pattern.
    Regex(Regex),
    /// Matches using a custom function.
    Fn(MatcherFn),
}

impl TokenMatcher {
    /// Attempts to match the token at the beginning of the input string.
    ///
    /// # Returns
    /// - `Some(usize)` with the length of the matched token if successful
    /// - `None` if no match is found
    #[must_use]
    pub fn matches(&self, input: &str) -> Option<usize> {
        match self {
            Self::Literal(literal) => input.starts_with(literal).then_some(literal.len()),
            Self::Regex(regex) => regex.find(input).map(|m| m.len()),
            Self::Fn(closure) => closure(input),
        }
    }
}

impl From<String> for TokenMatcher {
    fn from(literal: String) -> Self {
        Self::Literal(literal)
    }
}

impl From<&str> for TokenMatcher {
    fn from(literal: &str) -> Self {
        Self::Literal(literal.into())
    }
}

impl From<Regex> for TokenMatcher {
    fn from(regex: Regex) -> Self {
        Self::Regex(regex)
    }
}

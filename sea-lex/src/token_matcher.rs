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
//! let lit_matcher = LiteralMatcher::new("fn");
//! assert_eq!(lit_matcher.matches("fn fn"), Some(2));
//! assert_eq!(lit_matcher.matches("function fn"), None);
//!
//! let regex_matcher = RegexMatcher::new(Regex::new(r"\d+")?);
//! assert_eq!(regex_matcher.matches("123abc"), Some(3));
//! assert_eq!(regex_matcher.matches("abc123"), None);
//! # Ok(())
//! # };
//! ```

use regex::Regex;

/// A trait for types that can match tokens at the beginning of an input string.
pub trait TokenMatcher {
    /// Attempts to match the token at the beginning of the input string.
    ///
    /// # Returns
    /// - `Some(usize)` with the length of the matched token if successful.
    /// - `None` if no match is found.
    fn matches(&self, input: &str) -> Option<usize>;
}

/// Matches tokens using exact string comparison.
pub struct LiteralMatcher {
    /// The literal string to match against.
    pub literal: String,
}

impl LiteralMatcher {
    /// Creates a new `LiteralMatcher` for the given string.
    ///
    /// # Arguments
    /// * `literal` - The exact string to match
    #[must_use]
    pub fn new<S: Into<String>>(literal: S) -> Self {
        Self {
            literal: literal.into(),
        }
    }
}

impl<S> From<S> for LiteralMatcher
where
    S: Into<String>,
{
    fn from(literal: S) -> Self {
        Self::new(literal)
    }
}

impl TokenMatcher for LiteralMatcher {
    fn matches(&self, input: &str) -> Option<usize> {
        input
            .starts_with(&self.literal)
            .then_some(self.literal.len())
    }
}

/// Matches tokens using regular expressions.
pub struct RegexMatcher {
    /// The compiled regular expression to match against.
    pub regex: Regex,
}

impl RegexMatcher {
    /// Creates a new `RegexMatcher` for the given pattern.
    ///
    /// # Arguments
    /// * `regex` - The regular expression to match.
    #[must_use]
    pub const fn new(regex: Regex) -> Self {
        Self { regex }
    }
}

impl From<Regex> for RegexMatcher {
    fn from(regex: Regex) -> Self {
        Self::new(regex)
    }
}

impl TokenMatcher for RegexMatcher {
    fn matches(&self, input: &str) -> Option<usize> {
        self.regex
            .find(input)
            .and_then(|m| if m.start() == 0 { Some(m.end()) } else { None })
    }
}

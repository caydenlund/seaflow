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

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn test_literal_matcher() {
        let matcher = TokenMatcher::Literal("fn".into());

        assert_eq!(matcher.matches("fn"), Some(2));
        assert_eq!(matcher.matches("fn "), Some(2));
        assert_eq!(matcher.matches("fn foo"), Some(2));

        assert_eq!(matcher.matches("function"), None);
        assert_eq!(matcher.matches("nope"), None);
        assert_eq!(matcher.matches(""), None);
        assert_eq!(matcher.matches("f"), None);
    }

    #[test]
    fn test_regex_matcher() {
        let matcher = TokenMatcher::Regex(Regex::new(r"^\d+").unwrap());

        assert_eq!(matcher.matches("123"), Some(3));
        assert_eq!(matcher.matches("456abc"), Some(3));
        assert_eq!(matcher.matches("0 "), Some(1));

        assert_eq!(matcher.matches("abc123"), None);
        assert_eq!(matcher.matches(""), None);
    }

    #[test]
    fn test_custom_fn_matcher() {
        let matcher = TokenMatcher::Fn(Box::new(|input| {
            if input.starts_with("//") {
                input.find('\n').or(Some(input.len()))
            } else {
                None
            }
        }));

        assert_eq!(matcher.matches("// comment"), Some(10));
        assert_eq!(matcher.matches("//"), Some(2));
        assert_eq!(matcher.matches("//\nrest"), Some(2));

        assert_eq!(matcher.matches("not a comment"), None);
        assert_eq!(matcher.matches(""), None);
    }

    #[test]
    fn test_from_string_conversions() {
        let from_string = TokenMatcher::from("test".to_string());
        assert_eq!(from_string.matches("test"), Some(4));

        let from_str = TokenMatcher::from("test");
        assert_eq!(from_str.matches("test"), Some(4));
    }

    #[test]
    fn test_from_regex_conversion() {
        let regex = Regex::new(r"^[a-z]+").unwrap();
        let matcher = TokenMatcher::from(regex);

        assert_eq!(matcher.matches("abc"), Some(3));
        assert_eq!(matcher.matches("ABC"), None);
    }

    #[test]
    fn test_edge_cases() {
        let empty_lit = TokenMatcher::Literal("".into());
        assert_eq!(empty_lit.matches(""), Some(0));
        assert_eq!(empty_lit.matches("abc"), Some(0));

        let empty_re = TokenMatcher::Regex(Regex::new(r"^").unwrap());
        assert_eq!(empty_re.matches(""), Some(0));
        assert_eq!(empty_re.matches("abc"), Some(0));

        let zero_fn = TokenMatcher::Fn(Box::new(|_| Some(0)));
        assert_eq!(zero_fn.matches(""), Some(0));
        assert_eq!(zero_fn.matches("abc"), Some(0));
    }

    #[test]
    fn test_multibyte_characters() {
        let matcher = TokenMatcher::Literal("ƒ".into()); // Latin small f with hook
        assert_eq!(matcher.matches("ƒoo"), Some(2)); // Takes 2 bytes in UTF-8
        assert_eq!(matcher.matches("foo"), None);

        let matcher = TokenMatcher::Regex(Regex::new(r"^[^\p{ASCII}]+").unwrap());
        assert_eq!(matcher.matches("ƒoo"), Some(2));
        assert_eq!(matcher.matches("foo"), None);
    }

    #[test]
    fn test_complex_regex_matcher() {
        let matcher = TokenMatcher::Regex(Regex::new(r"^0x[0-9a-fA-F]+").unwrap());

        assert_eq!(matcher.matches("0x1a3F"), Some(6));
        assert_eq!(matcher.matches("0xG123"), None);
        assert_eq!(matcher.matches("x123"), None);
    }
}

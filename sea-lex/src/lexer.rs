//! sea-lex/lexer - Lexer module for tokenizing source code.
//!
//! This module provides a [`Lexer`] struct that can be used to convert source code
//! into a sequence of [`Token`]s. The lexer uses a set of matchers to identify token
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
//!     Token::new(EquationToken::Integer(1), "1", 0),
//!     Token::new(EquationToken::Plus, "+", 2),
//!     Token::new(EquationToken::Integer(23), "23", 4),
//!     Token::new(EquationToken::Minus, "-", 7),
//!     Token::new(EquationToken::Integer(45), "45", 9),
//!     Token::new(EquationToken::Asterisk, "*", 12),
//!     Token::new(EquationToken::Integer(6), "6", 14),
//!     Token::new(EquationToken::Slash, "/", 16),
//!     Token::new(EquationToken::Integer(789), "789", 18),
//! ];
//! assert_eq!(tokens, Ok(expected));
//! ```

use crate::LexerError;
use crate::Token;
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
    /// - `Ok(Vec<Token<T>>)` with the parsed tokens on success.
    /// - `Err(LexerError)` if lexing fails.
    ///
    /// # Errors
    /// Returns an error if unmatched text (an illegal character sequence) is encountered.
    ///
    /// # Behavior
    /// - Processes the input string from start to end.
    /// - For each position, tries matchers in order until a match is found.
    /// - Stops when no more tokens can be matched.
    pub fn lex(&self, source: &str) -> Result<Vec<Token<T>>, LexerError> {
        let mut tokens = Vec::new();
        let mut remaining = source;
        let mut position = 0;

        'outer: while !remaining.is_empty() {
            for (creator, matcher) in &self.matchers {
                if let Some(len) = matcher.matches(remaining) {
                    let matched_text = &remaining[..len];
                    remaining = &remaining[len..];

                    if let Some(typ) = creator.create(matched_text) {
                        tokens.push(Token::new(typ, matched_text, position));
                    }

                    position += len;
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

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum TestToken {
        Number(i64),
        Ident(String),
        Plus,
        Minus,
        LParen,
        RParen,
    }

    impl TokenType for TestToken {
        fn matchers() -> Vec<(TokenCreator<Self>, TokenMatcher)> {
            vec![
                (
                    TokenCreator::Fn(Box::new(|s| Self::Number(s.parse().unwrap()))),
                    Regex::new(r"^\d+").unwrap().into(),
                ),
                (
                    TokenCreator::Fn(Box::new(|s| Self::Ident(s.to_string()))),
                    Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*").unwrap().into(),
                ),
                (Self::Plus.into(), "+".into()),
                (Self::Minus.into(), "-".into()),
                (Self::LParen.into(), "(".into()),
                (Self::RParen.into(), ")".into()),
                (TokenCreator::None, Regex::new(r"^\s+").unwrap().into()), // skip whitespace
            ]
        }
    }

    #[test]
    fn test_basic_arithmetic() {
        let lexer = TestToken::lexer();
        let tokens = lexer.lex("123 + 456 - 789").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::new(TestToken::Number(123), "123", 0),
                Token::new(TestToken::Plus, "+", 4),
                Token::new(TestToken::Number(456), "456", 6),
                Token::new(TestToken::Minus, "-", 10),
                Token::new(TestToken::Number(789), "789", 12),
            ]
        );
    }

    #[test]
    fn test_identifiers_and_numbers() {
        let lexer = TestToken::lexer();
        let tokens = lexer.lex("foo 42 bar_123").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::new(TestToken::Ident("foo".to_string()), "foo", 0),
                Token::new(TestToken::Number(42), "42", 4),
                Token::new(TestToken::Ident("bar_123".to_string()), "bar_123", 7),
            ]
        );
    }

    #[test]
    fn test_whitespace_handling() {
        let lexer = TestToken::lexer();
        let tokens = lexer.lex("  \t\n123 \t\n + \n\r 456 ").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::new(TestToken::Number(123), "123", 4),
                Token::new(TestToken::Plus, "+", 11),
                Token::new(TestToken::Number(456), "456", 16),
            ]
        );
    }

    #[test]
    fn test_parentheses() {
        let lexer = TestToken::lexer();
        let tokens = lexer.lex("(foo + 123)").unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::new(TestToken::LParen, "(", 0),
                Token::new(TestToken::Ident("foo".to_string()), "foo", 1),
                Token::new(TestToken::Plus, "+", 5),
                Token::new(TestToken::Number(123), "123", 7),
                Token::new(TestToken::RParen, ")", 10),
            ]
        );
    }

    #[test]
    fn test_empty_input() {
        let lexer = TestToken::lexer();
        let tokens = lexer.lex("").unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn test_unmatched_character() {
        let lexer = TestToken::lexer();
        let result = lexer.lex("123 @ 456");

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.position, 4);
        assert_eq!(err.unmatched, '@');
    }

    #[test]
    fn test_partial_match_followed_by_error() {
        let lexer = TestToken::lexer();
        let result = lexer.lex("123 + $456");

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.position, 6);
        assert_eq!(err.unmatched, '$');
    }

    #[test]
    fn test_skipped_tokens_not_in_output() {
        let lexer = TestToken::lexer();
        let tokens = lexer.lex("  123   ").unwrap();

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::new(TestToken::Number(123), "123", 2));
    }

    #[test]
    fn test_token_positions_are_correct() {
        let lexer = TestToken::lexer();
        let tokens = lexer.lex("abc 123 ( x )").unwrap();

        assert_eq!(tokens[0].position, 0); // abc
        assert_eq!(tokens[1].position, 4); // 123
        assert_eq!(tokens[2].position, 8); // (
        assert_eq!(tokens[3].position, 10); // x
        assert_eq!(tokens[4].position, 12); // )
    }
}

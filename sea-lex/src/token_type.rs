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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{TokenCreator, TokenMatcher};
    use regex::Regex;

    // Define a test token type
    #[derive(Debug, Clone, PartialEq, Eq)]
    enum TestToken {
        Number(i64),
        Word(String),
        Plus,
        Minus,
    }

    impl TokenType for TestToken {
        fn matchers() -> Vec<(TokenCreator<Self>, TokenMatcher)> {
            vec![
                (
                    TokenCreator::Fn(Box::new(|s| TestToken::Number(s.parse().unwrap()))),
                    Regex::new(r"^\d+").unwrap().into(),
                ),
                (
                    TokenCreator::Fn(Box::new(|s| TestToken::Word(s.to_string()))),
                    Regex::new(r"^[a-zA-Z]+").unwrap().into(),
                ),
                (TestToken::Plus.into(), "+".into()),
                (TestToken::Minus.into(), "-".into()),
                (TokenCreator::None, Regex::new(r"^\s+").unwrap().into()),
            ]
        }
    }

    #[test]
    fn test_lexer_generation() {
        let lexer = TestToken::lexer();
        assert_eq!(lexer.matchers.len(), 5);
    }

    #[test]
    fn test_matcher_priority_order() {
        let matchers = TestToken::matchers();

        // Verify order matches our definition
        match &matchers[0].1 {
            TokenMatcher::Regex(re) => assert_eq!(re.as_str(), r"^\d+"),
            _ => panic!("First matcher should be number regex"),
        }

        match &matchers[1].1 {
            TokenMatcher::Regex(re) => assert_eq!(re.as_str(), r"^[a-zA-Z]+"),
            _ => panic!("Second matcher should be word regex"),
        }

        match &matchers[2].1 {
            TokenMatcher::Literal(s) => assert_eq!(s, "+"),
            _ => panic!("Third matcher should be plus literal"),
        }
    }

    #[test]
    fn test_token_creation_via_matchers() {
        let lexer = TestToken::lexer();
        let tokens = lexer.lex("123 + abc - 42").unwrap();

        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0].typ, TestToken::Number(123));
        assert_eq!(tokens[1].typ, TestToken::Plus);
        assert_eq!(tokens[2].typ, TestToken::Word("abc".to_string()));
        assert_eq!(tokens[3].typ, TestToken::Minus);
        assert_eq!(tokens[4].typ, TestToken::Number(42));
    }

    #[test]
    fn test_whitespace_skipping() {
        let lexer = TestToken::lexer();
        let tokens = lexer.lex("  123  \t\n+  ").unwrap();

        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].typ, TestToken::Number(123));
        assert_eq!(tokens[1].typ, TestToken::Plus);
    }

    #[test]
    fn test_mixed_content_lexing() {
        let lexer = TestToken::lexer();
        let tokens = lexer.lex("42apples+3bananas-7oranges").unwrap();

        assert_eq!(tokens.len(), 8);
        assert_eq!(tokens[0].typ, TestToken::Number(42));
        assert_eq!(tokens[1].typ, TestToken::Word("apples".to_string()));
        assert_eq!(tokens[2].typ, TestToken::Plus);
        assert_eq!(tokens[3].typ, TestToken::Number(3));
        assert_eq!(tokens[4].typ, TestToken::Word("bananas".to_string()));
        assert_eq!(tokens[5].typ, TestToken::Minus);
        assert_eq!(tokens[6].typ, TestToken::Number(7));
        assert_eq!(tokens[7].typ, TestToken::Word("oranges".to_string()));
    }

    #[test]
    fn test_lexer_error_on_unmatched() {
        let lexer = TestToken::lexer();
        let result = lexer.lex("123 $ 456");

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.position, 4);
        assert_eq!(err.unmatched, '$');
    }

    #[test]
    fn test_token_type_implementation() {
        let matchers = TestToken::matchers();
        assert!(matches!(matchers[0].0, TokenCreator::Fn(_)));
        assert!(matches!(matchers[2].0, TokenCreator::Cloned(_)));
        assert!(matches!(matchers[4].0, TokenCreator::None));
    }
}

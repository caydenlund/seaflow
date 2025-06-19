//! sea-lex/token-creator - A generic token creation system.
//!
//! This module provides the [`TokenCreator`] enum which abstracts different ways to
//! generate tokens of type `T`:
//! - From a cloneable source value
//! - Via a transformation function
//! - Explicit no-op case
//!
//! # Examples
//! ```
//! # use sea_lex::*;
//! #[derive(Debug, Clone, PartialEq, Eq)]
//! enum MyToken {
//!     WithContents(String),
//!     Static,
//! }
//! impl TokenType for MyToken {
//!     fn matchers() -> Vec<(TokenCreator<Self>, TokenMatcher)> { Vec::new() }
//! }
//!
//! // Create from a cloneable value
//! let creator = TokenCreator::from(MyToken::Static);
//! assert_eq!(creator.create("..."), Some(MyToken::Static));
//!
//! // Create via transformation function
//! let creator = TokenCreator::Fn(Box::new(|s| MyToken::WithContents(s.to_string().to_uppercase())));
//! assert_eq!(creator.create("test"), Some(MyToken::WithContents("TEST".to_string())));
//!
//! // No token creation
//! let creator = TokenCreator::<MyToken>::None;
//! assert_eq!(creator.create("anything"), None);
//! ```

use crate::TokenType;

/// A generic enum for creating tokens from different sources.
///
/// This provides flexibility in token creation by supporting the following:
/// - Cloning an existing token
/// - Generating a token via a closure
/// - Representing no token creation (e.g., for whitespace or comments)
///
/// # Type Parameters
/// - `T`: The type of token to be created.
pub enum TokenCreator<T>
where
    T: TokenType,
{
    /// Creates tokens by cloning an existing value of type `T`.
    Cloned(T),
    /// Creates tokens by invoking a boxed closure that takes a reference
    /// to the matched contents and returns a value of type `T`.
    ///
    /// The closure must be `Send`, `Sync`, and `'static`.
    Fn(Box<dyn Fn(&str) -> T + Send + Sync + 'static>),
    /// Doesn't create a new token.
    ///
    /// This is used for comments, whitespace, and anything else where
    /// you wouldn't want to emit a new token upon a match.
    None,
}

impl<T> TokenCreator<T>
where
    T: TokenType,
{
    /// Creates a token based on the variant and provided contents.
    ///
    /// # Arguments
    /// - `contents` - The input string used for token creation (when using `Fn` variant).
    ///
    /// # Returns
    /// `Some(T)` when a token is created.
    pub fn create(&self, contents: &str) -> Option<T>
    where
        T: Clone,
    {
        match self {
            Self::Cloned(token) => Some(token.clone()),
            Self::Fn(f) => Some(f(contents)),
            Self::None => None,
        }
    }
}

impl<T> From<T> for TokenCreator<T>
where
    T: TokenType,
{
    /// Converts a value of type `T` into a `TokenCreator::Cloned` variant.
    ///
    /// This provides a convenient way to create a [`TokenCreator`] from any cloneable type.
    ///
    /// # Arguments
    /// * `token` - The value to be wrapped in `TokenCreator::Cloned`.
    ///
    /// # Returns
    /// A `TokenCreator` that will clone the input value when `create` is called.
    fn from(token: T) -> Self {
        Self::Cloned(token)
    }
}

#[cfg(test)]
mod tests {
    use crate::TokenMatcher;

    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum TestToken {
        Static,
        Number(i64),
        Text(String),
    }

    impl TokenType for TestToken {
        fn matchers() -> Vec<(TokenCreator<Self>, TokenMatcher)> {
            Vec::new()
        }
    }

    #[test]
    fn test_cloned_variant() {
        let creator = TokenCreator::Cloned(TestToken::Static);
        assert_eq!(creator.create("anything"), Some(TestToken::Static));

        let num_creator = TokenCreator::Cloned(TestToken::Number(42));
        assert_eq!(num_creator.create("ignored"), Some(TestToken::Number(42)));
    }

    #[test]
    fn test_fn_variant() {
        let creator = TokenCreator::Fn(Box::new(|s| TestToken::Number(s.parse().unwrap())));

        assert_eq!(creator.create("123"), Some(TestToken::Number(123)));
        assert_eq!(creator.create("0"), Some(TestToken::Number(0)));
    }

    #[test]
    fn test_fn_variant_with_string() {
        let creator = TokenCreator::Fn(Box::new(|s| TestToken::Text(s.to_uppercase())));

        assert_eq!(
            creator.create("hello"),
            Some(TestToken::Text("HELLO".to_string()))
        );
        assert_eq!(
            creator.create("world"),
            Some(TestToken::Text("WORLD".to_string()))
        );
    }

    #[test]
    fn test_none_variant() {
        let creator = TokenCreator::<TestToken>::None;
        assert_eq!(creator.create("anything"), None);
        assert_eq!(creator.create(""), None);
    }

    #[test]
    fn test_from_impl() {
        let creator: TokenCreator<TestToken> = TestToken::Static.into();
        assert_eq!(creator.create(""), Some(TestToken::Static));

        let creator: TokenCreator<TestToken> = TestToken::Number(99).into();
        assert_eq!(creator.create("ignored"), Some(TestToken::Number(99)));
    }

    #[test]
    fn test_cloned_variant_with_multiple_calls() {
        let creator = TokenCreator::Cloned(TestToken::Static);
        assert_eq!(creator.create("first"), Some(TestToken::Static));
        assert_eq!(creator.create("second"), Some(TestToken::Static));
        assert_eq!(creator.create("third"), Some(TestToken::Static));
    }

    #[test]
    fn test_fn_variant_with_different_inputs() {
        let creator = TokenCreator::Fn(Box::new(|s| TestToken::Text(format!("processed: {}", s))));

        assert_eq!(
            creator.create("input1"),
            Some(TestToken::Text("processed: input1".to_string()))
        );
        assert_eq!(
            creator.create("input2"),
            Some(TestToken::Text("processed: input2".to_string()))
        );
    }

    #[test]
    fn test_empty_string_input() {
        let fn_creator = TokenCreator::Fn(Box::new(|s| TestToken::Text(s.to_string())));
        assert_eq!(fn_creator.create(""), Some(TestToken::Text("".to_string())));

        let cloned_creator = TokenCreator::Cloned(TestToken::Static);
        assert_eq!(cloned_creator.create(""), Some(TestToken::Static));
    }
}

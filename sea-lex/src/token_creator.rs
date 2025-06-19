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
//!     fn matchers() -> Vec<Matcher<Self>> { Vec::new() }
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

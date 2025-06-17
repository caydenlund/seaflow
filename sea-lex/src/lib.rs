//! sea-lex - seaflow lexer component

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::missing_docs_in_private_items,
    clippy::nursery,
    clippy::pedantic,
    missing_docs,
    rustdoc::all
)]

use regex::Regex;

pub enum TokenCreator<T> {
    Cloned(T),
    Fn(Box<dyn Fn(&str) -> T + Send + Sync + 'static>),
    None,
}

impl<T> TokenCreator<T> {
    pub fn create(&self, matched_str: &str) -> Option<T>
    where
        T: Clone, // `T` must be cloneable for the `Cloned` variant
    {
        match self {
            TokenCreator::Cloned(token) => Some(token.clone()),
            TokenCreator::Fn(f) => Some(f(matched_str)),
            TokenCreator::None => None,
        }
    }
}

impl<T> From<T> for TokenCreator<T>
where
    T: Clone,
{
    fn from(token: T) -> Self {
        TokenCreator::Cloned(token)
    }
}

pub enum MatcherPattern {
    Literal(String),
    Regex(Regex),
}

impl MatcherPattern {
    pub fn matches(&self, text: &str) -> Option<usize> {
        match self {
            MatcherPattern::Literal(s) => text.starts_with(s).then_some(s.len()),
            MatcherPattern::Regex(r) => r.find(text).map(|m| m.end()),
        }
    }
}

// Implement From for convenience
impl From<&str> for MatcherPattern {
    fn from(s: &str) -> Self {
        MatcherPattern::Literal(s.to_string())
    }
}

impl From<Regex> for MatcherPattern {
    fn from(regex: Regex) -> Self {
        MatcherPattern::Regex(regex)
    }
}

pub struct Matcher<T> {
    creator: TokenCreator<T>,
    pattern: MatcherPattern,
}

impl<T, C> From<(C, Regex)> for Matcher<T>
where
    C: Into<TokenCreator<T>>,
{
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
{
    fn from((creator, literal): (C, &str)) -> Self {
        Self {
            creator: creator.into(),
            pattern: MatcherPattern::Literal(literal.to_string()),
        }
    }
}

pub trait TokenType: Sized + Clone {
    fn matchers() -> Vec<Matcher<Self>>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Integer(i64),
    Identifier(String),
    LParen,
    RParen,
    Asterisk,
    Slash,
    Plus,
    Minus,
}

impl TokenType for Token {
    fn matchers() -> Vec<Matcher<Self>> {
        vec![
            (
                TokenCreator::Fn(Box::new(|c| Self::Integer(c.parse().unwrap()))),
                Regex::new(r"^\d+").unwrap(),
            )
                .into(),
            (
                TokenCreator::Fn(Box::new(|c| Self::Identifier(c.to_string()))),
                Regex::new(r"^[a-zA-Z_]+").unwrap(),
            )
                .into(),
            (Self::LParen, "(").into(),
            (Self::RParen, ")").into(),
            (Self::Asterisk, "*").into(),
            (Self::Slash, "/").into(),
            (Self::Plus, "+").into(),
            (Self::Minus, "-").into(),
            (TokenCreator::None, Regex::new(r"^\s+").unwrap()).into(),
        ]
    }
}

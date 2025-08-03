//! Lexer implementation for sea-lex

use crate::{LexError, TokenInfo};
use regex::Regex;
use std::sync::Arc;

/// A compiled lexer for a specific token type
pub struct Lexer<T> {
    /// The input string being lexed
    input: String,
    /// The current position in the input
    position: usize,
    /// The compiled token matchers
    matchers: Vec<(TokenMatcher, TokenCreator<T>)>,
    /// The compiled skip patterns
    skip_patterns: Vec<TokenMatcher>,
}

/// A compiled token matcher
enum TokenMatcher {
    /// A regular expression matcher
    RegexMatcher {
        /// The regex pattern to match
        pattern: Regex,
    },
    /// A literal string matcher
    LiteralMatcher {
        /// The literal string to match
        pattern: String,
    },
}

/// Function to create a token from matched text
pub enum TokenCreator<T> {
    /// Create a unit variant (no data)
    Unit(T),
    /// Create a variant by calling a parser on the matched text
    Parser(std::sync::Arc<dyn Fn(&str, usize) -> Result<T, crate::LexError> + Send + Sync>),
    /// Skip this match (don't emit a token)
    Skip,
}

impl<T: Clone> Lexer<T> {
    /// Create a new lexer with the given input and matchers
    ///
    /// # Errors
    ///
    /// Returns a `LexError` if any of the provided regex patterns are invalid
    pub fn new(
        input: impl Into<String>,
        matchers: Vec<(TokenCreator<T>, &str, bool)>, // bool indicates if regex
        skip_patterns: Vec<(&str, bool)>,             // bool indicates if regex
    ) -> Result<Self, LexError> {
        let input = input.into();

        let compiled_matchers = matchers
            .into_iter()
            .map(|(creator, pattern, is_regex)| {
                TokenMatcher::try_new(pattern, is_regex).map(|matcher| (matcher, creator))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let compiled_skip_patterns = skip_patterns
            .into_iter()
            .map(|(pattern, is_regex)| TokenMatcher::try_new(pattern, is_regex))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            input,
            position: 0,
            matchers: compiled_matchers,
            skip_patterns: compiled_skip_patterns,
        })
    }

    /// Get the next token from the input
    pub fn next_token(&mut self) -> Option<Result<TokenInfo<T>, LexError>> {
        'retry_skip: loop {
            if self.position >= self.input.len() {
                return None;
            }

            let remaining = &self.input[self.position..];

            // Try skip patterns first
            for skip_pattern in &self.skip_patterns {
                if let Some(len) = skip_pattern.try_match(remaining) {
                    self.position += len;
                    continue 'retry_skip;
                }
            }
            break;
        }

        let remaining = &self.input[self.position..];

        // Try token matchers
        for (matcher, creator) in &self.matchers {
            if let Some(match_len) = matcher.try_match(remaining) {
                let start = self.position;
                let end = self.position + match_len;
                let text = &self.input[start..end];
                self.position = end;

                match creator {
                    TokenCreator::Unit(token) => {
                        return Some(Ok(TokenInfo::new(token.clone(), text, start, end)));
                    }
                    TokenCreator::Parser(parser) => {
                        return Some(
                            parser(text, start).map(|token| TokenInfo::new(token, text, start, end)),
                        );
                    }
                    TokenCreator::Skip => {
                        break; // Continue to next iteration to skip this match
                    }
                }
            }
        }

        // No pattern matched
        Some(Err(LexError::UnexpectedChar {
            position: self.position,
            character: remaining.chars().next().unwrap_or_default(),
        }))
    }

    /// Collect all tokens into a vector
    ///
    /// # Errors
    ///
    /// Returns a `LexError` if the input contains unrecognized characters
    pub fn collect(mut self) -> Result<Vec<TokenInfo<T>>, LexError> {
        let mut tokens = Vec::new();
        while let Some(result) = self.next_token() {
            tokens.push(result?);
        }
        Ok(tokens)
    }
}

impl<T: Clone> Iterator for Lexer<T> {
    type Item = Result<TokenInfo<T>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_token()
    }
}

impl TokenMatcher {
    /// Tries to create a new [`TokenMatcher`] from the given pattern
    ///
    /// # Errors
    ///
    /// Returns a `LexError` if any of the provided regex patterns are invalid
    pub fn try_new(pattern: &str, is_regex: bool) -> Result<Self, LexError> {
        if is_regex {
            // Add `^` if not present
            if pattern.starts_with('^') {
                Regex::new(pattern)
            } else {
                Regex::new(&format!("^{pattern}"))
            }
            .map(|pattern| Self::RegexMatcher { pattern })
            .map_err(|error| LexError::InvalidRegex {
                pattern: pattern.into(),
                error,
            })
        } else {
            Ok(Self::LiteralMatcher {
                pattern: pattern.into(),
            })
        }
    }

    /// Reports whether this pattern matches the given text,
    /// and returns the length of the match if successful
    pub fn try_match(&self, text: &str) -> Option<usize> {
        match self {
            Self::RegexMatcher { pattern } => pattern.find(text).map(|m| m.len()),
            Self::LiteralMatcher { pattern } => text.starts_with(pattern).then_some(pattern.len()),
        }
    }
}

impl<T: Clone> Clone for TokenCreator<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Unit(token) => Self::Unit(token.clone()),
            Self::Parser(parser) => Self::Parser(Arc::clone(parser)),
            Self::Skip => Self::Skip,
        }
    }
}

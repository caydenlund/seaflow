//! Lexer implementation for sea-lex

use crate::{LexError, TokenInfo};
use regex::Regex;

/// A compiled lexer for a specific token type
pub struct Lexer<T> {
    /// The input string being lexed
    input: String,
    /// The current position in the input
    position: usize,
    /// The compiled token matchers
    matchers: Vec<TokenMatcher<T>>,
    /// The compiled skip patterns
    skip_patterns: Vec<Regex>,
}

/// A compiled token matcher
struct TokenMatcher<T> {
    /// The regex pattern to match
    pattern: Regex,
    /// Function to create the token from matched text
    creator: TokenCreator<T>,
}

/// Function to create a token from matched text
pub enum TokenCreator<T> {
    /// Create a unit variant (no data)
    Unit(T),
    /// Create a variant by calling a function on the matched text
    Function(fn(&str) -> T),
    /// Skip this match (don't emit a token)
    Skip,
}

impl<T: Clone> Clone for TokenCreator<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Unit(token) => Self::Unit(token.clone()),
            Self::Function(func) => Self::Function(*func),
            Self::Skip => Self::Skip,
        }
    }
}

impl<T: Clone> Lexer<T> {
    /// Create a new lexer with the given input and matchers
    pub fn new(
        input: impl Into<String>,
        matchers: Vec<(TokenCreator<T>, &str, bool)>, // bool indicates if regex
        skip_patterns: Vec<(&str, bool)>, // bool indicates if regex
    ) -> Result<Self, LexError> {
        let input = input.into();
        
        let compiled_matchers = matchers
            .into_iter()
            .map(|(creator, pattern, is_regex)| {
                let regex = if is_regex {
                    // It's a regex pattern - add ^ if not present
                    if pattern.starts_with('^') {
                        Regex::new(pattern)
                    } else {
                        Regex::new(&format!("^{}", pattern))
                    }
                } else {
                    // It's a literal string - escape it
                    Regex::new(&format!("^{}", regex::escape(pattern)))
                };
                
                regex
                    .map(|pattern| TokenMatcher { pattern, creator })
                    .map_err(|e| LexError::new(0, format!("Invalid regex pattern '{}': {}", pattern, e)))
            })
            .collect::<Result<Vec<_>, _>>()?;

        let compiled_skip_patterns = skip_patterns
            .into_iter()
            .map(|(pattern, is_regex)| {
                let regex = if is_regex {
                    if pattern.starts_with('^') {
                        Regex::new(pattern)
                    } else {
                        Regex::new(&format!("^{}", pattern))
                    }
                } else {
                    Regex::new(&format!("^{}", regex::escape(pattern)))
                };
                regex
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| LexError::new(0, format!("Invalid skip pattern: {}", e)))?;

        Ok(Self {
            input,
            position: 0,
            matchers: compiled_matchers,
            skip_patterns: compiled_skip_patterns,
        })
    }

    /// Get the next token from the input
    pub fn next(&mut self) -> Option<Result<TokenInfo<T>, LexError>> {
        loop {
            if self.position >= self.input.len() {
                return None;
            }

            let remaining = &self.input[self.position..];

            // Try skip patterns first
            let mut skipped = false;
            for skip_pattern in &self.skip_patterns {
                if let Some(mat) = skip_pattern.find(remaining) {
                    self.position += mat.end();
                    skipped = true;
                    break;
                }
            }

            if skipped {
                continue;
            }

            // Try token matchers
            for matcher in &self.matchers {
                if let Some(mat) = matcher.pattern.find(remaining) {
                    let text = mat.as_str();
                    let start = self.position;
                    let end = self.position + mat.end();
                    self.position = end;

                    match &matcher.creator {
                        TokenCreator::Unit(token) => {
                            return Some(Ok(TokenInfo::new(
                                token.clone(),
                                text,
                                start,
                                end,
                            )));
                        }
                        TokenCreator::Function(func) => {
                            let token = func(text);
                            return Some(Ok(TokenInfo::new(token, text, start, end)));
                        }
                        TokenCreator::Skip => {
                            break; // Continue to next iteration to skip this match
                        }
                    }
                }
            }

            // No pattern matched
            return Some(Err(LexError::new(
                self.position,
                format!("Unexpected character: '{}'", remaining.chars().next().unwrap_or('\0')),
            )));
        }
    }

    /// Collect all tokens into a vector
    pub fn collect(mut self) -> Result<Vec<TokenInfo<T>>, LexError> {
        let mut tokens = Vec::new();
        while let Some(result) = self.next() {
            tokens.push(result?);
        }
        Ok(tokens)
    }
}

impl<T: Clone> Iterator for Lexer<T> {
    type Item = Result<TokenInfo<T>, LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}
